use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Re-export the test module to run integration tests
#[path = "../tests/integration_test.rs"]
mod integration_test;