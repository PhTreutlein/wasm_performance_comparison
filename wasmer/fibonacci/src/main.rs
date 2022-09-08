use std::time::Instant;
use wasmer::{imports, Instance, Module, Store, Value, Universal};
use wasmer_compiler_llvm::LLVM;

fn main() -> anyhow::Result<()> {
    let wasm_bytes = std::fs::read("./fibonacci_bg.wasm")?;

    // for LLVM compiler
    let compiler = LLVM::new();
    let store = Store::new(&Universal::new(compiler).engine());

    // for default cranelift compiler:
    // let store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    // input data for n-th fibonacci number here
    let n = Value::I32(20);

    let fibonacci_recursive = instance.exports.get_function("fibonacci_recursive")?;
    let start = Instant::now();
    let result = fibonacci_recursive.call(&[n])?;
    let duration = start.elapsed();
    println!("Result: {:?}, Time: {:?}", result, duration);

    Ok(())
}
