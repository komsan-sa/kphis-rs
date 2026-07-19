use wasm_bindgen_test::*;

use kphis_ui_core::token::get_claim_encoded_key_public;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// reverse of test_gen_new_access_token_and_get_claim_public()
#[wasm_bindgen_test]
pub fn test_get_claim_from_token_and_key() {
    let token = "v4.public.eyJzdWIiOiIwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDBaNyIsIm5hbWUiOiJzb21lb25lIiwiYWN0IjoiYWNjZXNzIiwiaWF0IjoxNzg0NDYwMzIxLCJleHAiOjE3ODQ0NjA1MDEsInJleHAiOjE3ODQ0NjIxMjF9XxRlNOnvqW2DrtekUv1Py6crqxf2TeBhVspZ8J0eVrpoh7AROSCrGDxmGCKnPy5egXzbd-DrdnGMc92pJCmaDA";
    let key = "qNAd9ZKljQ99tNKbbt78iJ1Kx_-oADy3MI-oGpJOtxU";
    let iat = 1784460321u64;

    let claims = get_claim_encoded_key_public(token, key).unwrap();

    assert_eq!(claims.sub, String::from("000000000000000000000000Z7")); // Ulid(999).to_string());
    assert_eq!(claims.name, String::from("someone"));
    assert_eq!(claims.act, String::from("access"));
    assert_eq!(claims.iat, iat);
    assert_eq!(claims.exp, iat + (3 * 60));
    assert_eq!(claims.rexp, iat + (30 * 60));
}
