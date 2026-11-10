//! `Op` の木から LLVM IR を生成する。
//!
//! 生成されるプログラムの形:
//!
//! ```llvm
//! @tape = internal global [N x i8] zeroinitializer
//! declare i32 @putchar(i32)
//! declare i32 @getchar()
//! define i32 @main() {
//!   %ptr = alloca i64          ; テープ上の位置 (mem2reg でレジスタ化される)
//!   ...
//!   ret i32 0
//! }
//! ```

use inkwell::basic_block::BasicBlock;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::TargetMachine;
use inkwell::types::{ArrayType, IntType};
use inkwell::values::{FunctionValue, GlobalValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};

use crate::interpreter::TAPE_SIZE;
use crate::parser::Op;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    i8_type: IntType<'ctx>,
    i32_type: IntType<'ctx>,
    i64_type: IntType<'ctx>,
    tape_type: ArrayType<'ctx>,
    tape: GlobalValue<'ctx>,
    putchar: FunctionValue<'ctx>,
    getchar: FunctionValue<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        module.set_triple(&TargetMachine::get_default_triple());

        let i8_type = context.i8_type();
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();

        // テープ本体: ゼロ初期化された内部グローバル配列
        let tape_type = i8_type.array_type(TAPE_SIZE as u32);
        let tape = module.add_global(tape_type, Some(AddressSpace::default()), "tape");
        tape.set_linkage(Linkage::Internal);
        tape.set_initializer(&tape_type.const_zero());

        // libc の入出力関数
        let putchar = module.add_function(
            "putchar",
            i32_type.fn_type(&[i32_type.into()], false),
            Some(Linkage::External),
        );
        let getchar = module.add_function(
            "getchar",
            i32_type.fn_type(&[], false),
            Some(Linkage::External),
        );

        Self {
            context,
            module,
            builder,
            i8_type,
            i32_type,
            i64_type,
            tape_type,
            tape,
            putchar,
            getchar,
        }
    }

    /// `main` 関数を生成し、検証済みのモジュールを返す。
    pub fn compile(self, ops: &[Op]) -> Result<Module<'ctx>, String> {
        let main_fn = self
            .module
            .add_function("main", self.i32_type.fn_type(&[], false), None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);

        let ptr = self.builder.build_alloca(self.i64_type, "ptr").map_err(err)?;
        self.builder
            .build_store(ptr, self.i64_type.const_zero())
            .map_err(err)?;

        self.emit_ops(ops, main_fn, ptr).map_err(err)?;

        self.builder
            .build_return(Some(&self.i32_type.const_zero()))
            .map_err(err)?;

        self.module
            .verify()
            .map_err(|e| format!("LLVM module verification failed: {}", e.to_string()))?;
        Ok(self.module)
    }

    fn emit_ops(
        &self,
        ops: &[Op],
        func: FunctionValue<'ctx>,
        ptr: PointerValue<'ctx>,
    ) -> Result<(), BuilderError> {
        for op in ops {
            match op {
                Op::Add(n) => {
                    let cell = self.current_cell(ptr)?;
                    let v = self.builder.build_load(self.i8_type, cell, "v")?.into_int_value();
                    let n = self.i8_type.const_int(*n as u64, false);
                    let v = self.builder.build_int_add(v, n, "v")?;
                    self.builder.build_store(cell, v)?;
                }
                Op::Move(offset) => {
                    let idx = self.builder.build_load(self.i64_type, ptr, "idx")?.into_int_value();
                    // 負数は 2 の補数として渡す (sign_extend = true)
                    let off = self.i64_type.const_int(*offset as i64 as u64, true);
                    let idx = self.builder.build_int_add(idx, off, "idx")?;
                    self.builder.build_store(ptr, idx)?;
                }
                Op::Output => {
                    let cell = self.current_cell(ptr)?;
                    let v = self.builder.build_load(self.i8_type, cell, "v")?.into_int_value();
                    let c = self.builder.build_int_z_extend(v, self.i32_type, "c")?;
                    self.builder.build_call(self.putchar, &[c.into()], "")?;
                }
                Op::Input => {
                    // EOF (-1) のときはセルを変更しない (インタプリタと同じ挙動)
                    let cell = self.current_cell(ptr)?;
                    let old = self.builder.build_load(self.i8_type, cell, "old")?.into_int_value();
                    let c = self
                        .builder
                        .build_call(self.getchar, &[], "c")?
                        .try_as_basic_value()
                        .expect_basic("getchar returns i32")
                        .into_int_value();
                    let is_eof = self.builder.build_int_compare(
                        IntPredicate::EQ,
                        c,
                        self.i32_type.const_all_ones(),
                        "is_eof",
                    )?;
                    let new = self.builder.build_int_truncate(c, self.i8_type, "new")?;
                    let v = self.builder.build_select(is_eof, old, new, "v")?;
                    self.builder.build_store(cell, v)?;
                }
                Op::Loop(body) => {
                    let cond_bb = self.append_block(func, "loop.cond");
                    let body_bb = self.append_block(func, "loop.body");
                    let end_bb = self.append_block(func, "loop.end");

                    self.builder.build_unconditional_branch(cond_bb)?;

                    // while (tape[ptr] != 0)
                    self.builder.position_at_end(cond_bb);
                    let cell = self.current_cell(ptr)?;
                    let v = self.builder.build_load(self.i8_type, cell, "v")?.into_int_value();
                    let nonzero = self.builder.build_int_compare(
                        IntPredicate::NE,
                        v,
                        self.i8_type.const_zero(),
                        "nonzero",
                    )?;
                    self.builder.build_conditional_branch(nonzero, body_bb, end_bb)?;

                    self.builder.position_at_end(body_bb);
                    self.emit_ops(body, func, ptr)?;
                    self.builder.build_unconditional_branch(cond_bb)?;

                    self.builder.position_at_end(end_bb);
                }
            }
        }
        Ok(())
    }

    /// `&tape[ptr]` を計算する。
    fn current_cell(&self, ptr: PointerValue<'ctx>) -> Result<PointerValue<'ctx>, BuilderError> {
        let idx = self.builder.build_load(self.i64_type, ptr, "idx")?.into_int_value();
        let zero = self.i64_type.const_zero();
        // SAFETY: インデックスは BF プログラムが正しくテープ内に留まる前提。
        unsafe {
            self.builder.build_in_bounds_gep(
                self.tape_type,
                self.tape.as_pointer_value(),
                &[zero, idx],
                "cell",
            )
        }
    }

    fn append_block(&self, func: FunctionValue<'ctx>, name: &str) -> BasicBlock<'ctx> {
        self.context.append_basic_block(func, name)
    }
}

fn err(e: BuilderError) -> String {
    format!("LLVM builder error: {e}")
}
