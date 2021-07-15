Build
------------

.. code:: bash
    
    # 编译 WASM 字节码
    cd code
    cargo build --target wasm32-unknown-unknown --release
    cp ~/.cargo/target/wasm32-unknown-unknown/release/code.wasm ../
    
    cd ../
    wasm-gc code.wasm
    cargo run
