use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
}
