use wasm_bindgen_test::*;

use kphis_ui_core::token::get_claim_encoded_key_public;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// reverse of test_gen_new_access_token_and_get_claim_public()
#[wasm_bindgen_test]
pub fn test_get_claim_from_token_and_key() {
    let token = "v4.public.eyJzdWIiOiIwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDBaNyIsImFjdCI6ImFjY2VzcyIsImlhdCI6MTc4NDcyMTYwMywiZXhwIjoxNzg0NzIxNzgzfX4DNnBm3cQ4nq9wfwbVn7BOJADC4oDjJoI7GaQTY4JSpa8lluYXbfjd6CWQFwbE740kwPhWCoBSX1kZOP_x6A8";
    let key = "1OiEntO2nqhVXyCQYOSSi3Ku1AOxB0ER8j0Ha-M0Wm0";
    let iat = 1784721603u64;

    let claims = get_claim_encoded_key_public(token, key).unwrap();

    assert_eq!(claims.sub, String::from("000000000000000000000000Z7")); // Ulid(999).to_string());
    assert_eq!(claims.act, String::from("access"));
    assert_eq!(claims.iat, iat);
    assert_eq!(claims.exp, iat + (3 * 60));
}
