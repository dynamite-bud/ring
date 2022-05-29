use ring::{signature, test};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn signature_impl_test() {
    test::compile_time_assert_clone::<signature::Signature>();
    test::compile_time_assert_copy::<signature::Signature>();
    test::compile_time_assert_send::<signature::Signature>();
    test::compile_time_assert_sync::<signature::Signature>();
}
