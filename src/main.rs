

pub mod builtins;
pub mod runtime;


// NOTE: 已编译好的 WASM 字节码。
static WASM_CODE: &[u8] = include_bytes!("../code.wasm");


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builtins = runtime::load_global_builtins();
    let mut vm = runtime::Vm::new(builtins, WASM_CODE)?;

    println!("{:?}", vm.exec_test_fn_mul(10, 3) );

    println!("{:?}", vm.start() );

    Ok(())
}
