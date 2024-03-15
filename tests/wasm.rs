#[cfg(test)]

use wasm_bindgen_test::*;

/// Test the test
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
