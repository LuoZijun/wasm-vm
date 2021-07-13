#![cfg(target_arch = "wasm32")]


#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[no_mangle]
pub fn mul(a: u32, b: u32) -> u32 {
    extern "C" {
        fn fil_mul(a: u32, b: u32) -> u32;
    }

    // NOTE: 使用解释器宿主机器里面的 函数。
    unsafe { fil_mul(a, b) }
}


#[no_mangle]
pub fn fil_start() -> u32 {
    let a = 10i32;
    let b = 20i32;
    
    mul(10, add(a, b) as u32)
}