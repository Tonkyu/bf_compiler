mod codegen;
mod interpreter;
mod parser;

use std::path::{Path, PathBuf};
use std::{env, fs, process};

use inkwell::context::Context;

use codegen::CodeGen;
use interpreter::Interpreter;

fn usage(prog: &str) -> ! {
    eprintln!("Usage:");
    eprintln!("  {prog} <file.bf>                          interpret");
    eprintln!("  {prog} --emit-llvm <file.bf> [-o out.ll]  emit LLVM IR");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args[0].as_str();

    let mut emit_llvm = false;
    let mut output: Option<PathBuf> = None;
    let mut input: Option<PathBuf> = None;

    let mut it = args.iter().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--emit-llvm" => emit_llvm = true,
            "-o" => output = Some(it.next().unwrap_or_else(|| usage(prog)).into()),
            _ if input.is_none() => input = Some(arg.into()),
            _ => usage(prog),
        }
    }
    let input = input.unwrap_or_else(|| usage(prog));

    let code = fs::read_to_string(&input).unwrap_or_else(|e| {
        eprintln!("Error: Failed to read file '{}': {e}", input.display());
        process::exit(1);
    });

    let ops = parser::parse(&code).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        process::exit(1);
    });

    if emit_llvm {
        let output = output.unwrap_or_else(|| input.with_extension("ll"));
        if let Err(e) = emit_ir(&ops, &input, &output) {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    } else {
        Interpreter::new().run(&ops);
    }
}

fn emit_ir(ops: &[parser::Op], input: &Path, output: &Path) -> Result<(), String> {
    let context = Context::create();
    let module_name = input.file_stem().and_then(|s| s.to_str()).unwrap_or("bf");
    let module = CodeGen::new(&context, module_name).compile(ops)?;
    module
        .print_to_file(output)
        .map_err(|e| format!("failed to write '{}': {}", output.display(), e.to_string()))
}
