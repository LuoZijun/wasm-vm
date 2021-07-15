Build
------------

.. code:: bash
    
    cd code
    cargo build --target wasm32-unknown-unknown --release
    cp ~/.cargo/target/wasm32-unknown-unknown/release/code.wasm ../

    cd ../
    wasm-gc code.wasm
    cargo run
