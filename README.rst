
.. code::bash
    
    cd code
    cargo build --target wasm32-unknown-unknown --release
    cp ~/.cargo/target/wasm32-unknown-unknown/release/code.wasm ../
    
    cd ../
    cargo run
