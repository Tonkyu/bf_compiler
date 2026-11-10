# Brainfuck Interpreter / LLVM IR Compiler Written in Rust

## Requirements
- LLVM 20 (`brew install llvm`)
  - `.cargo/config.toml` で `LLVM_SYS_201_PREFIX=/opt/homebrew/opt/llvm` を指定済み

## How to Use

### Interpret
```
cargo run -- ./bf_src/hello_world.bf
```

### Dump Parsed Ops
```
cargo run -- --dump-ops ./bf_src/hello_world.bf
```
パース後の `Op` 列をループごとにインデントして表示する（実行はしない）

### Emit LLVM IR
```
cargo run -- --emit-llvm ./bf_src/hello_world.bf -o hello_world.ll
clang -O2 hello_world.ll -o hello_world
./hello_world
```
`-o` を省略すると入力ファイルの拡張子を `.ll` に変えたパスに出力する

## Notes
- テープ長は 300,000 セル（範囲外チェックなし）
- `,` で EOF を読んだ場合、セルの値は変更しない
