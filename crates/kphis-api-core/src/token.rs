use pasetors::{
    Public,
    keys::{AsymmetricPublicKey, AsymmetricSecretKey},
    token::UntrustedToken,
    version4::{PublicToken, V4},
};
use ulid::Ulid;

use kphis_model::{SERVER_ENTITY, claim::Claims};
use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, ErrorTitle, Source},
};

pub enum TokenType {
    Access,
    Refresh,
}
impl TokenType {
    pub fn string(self) -> String {
        match self {
            TokenType::Access => String::from("access"),
            TokenType::Refresh => String::from("refresh"),
        }
    }
}

/// create token with claim format = json(sub<Ulid>, name<String>, act<TokenType>, iat<u64>, exp<u64>)
pub fn gen_token_public(
    sub: u128,
    exp_minute: (u64, u64), // (access, refresh)
    old_rexp: Option<u64>,
    token_type: TokenType,
    key: &AsymmetricSecretKey<V4>,
) -> Result<String, AppError> {
    let (access_minute, refresh_minute) = exp_minute;
    let iat = get_timestamp_server()?;
    let aexp = iat + (access_minute * 60);
    let rexp = old_rexp.unwrap_or(iat + (refresh_minute * 60));
    let exp = match token_type {
        TokenType::Access => aexp,
        TokenType::Refresh => rexp,
    };
    let claims = Claims {
        sub: Ulid(sub).to_string(),
        act: token_type.string(),
        iat,
        exp,
    };
    let message = serde_json::to_vec(&claims).map_err(|e| Source::SerdeJson.to_error(500, e, "Generate Token"))?;
    let token = PublicToken::sign(key, &message, None, Some(SERVER_ENTITY.as_bytes())).map_err(|e| Source::Pasetors.to_error(500, e, "Generate Token"))?;

    Ok(token)
}

/// May error 401, 500
pub fn get_claim_and_verify_public(token: &str, public_key: &AsymmetricPublicKey<V4>) -> Result<Claims, AppError> {
    let claims = get_claim_public(token, public_key)?;

    let now_ts = get_timestamp_server()?;
    if claims.iat > now_ts || claims.exp < now_ts {
        return Err(AppError::app_401("Verify Token").with_title(ErrorTitle::Security));
    }

    Ok(claims)
}

pub fn get_claim_public(token: &str, public_key: &AsymmetricPublicKey<V4>) -> Result<Claims, AppError> {
    let untrusted_token = UntrustedToken::<Public, V4>::try_from(token).map_err(|_e| AppError::app_401("Verify Token"))?;
    let trusted_token = PublicToken::verify(public_key, &untrusted_token, None, Some(SERVER_ENTITY.as_bytes())).map_err(|_e| AppError::app_401("Verify Token"))?;

    let claims_string = trusted_token.payload();
    let claims = serde_json::from_str::<Claims>(claims_string).map_err(|_e| AppError::app_401("Verify Token").with_title(ErrorTitle::Security))?;

    Ok(claims)
}

#[cfg(test)]
mod tests {

    // use base64::{Engine, engine::general_purpose};
    use pasetors::keys::{AsymmetricKeyPair, Generate};

    use super::*;

    #[test]
    fn test_gen_new_access_token_and_get_claim_public() {
        let paseto = AsymmetricKeyPair::<V4>::generate().expect("Fail generating keypair");
        let token = gen_token_public(999, (3, 30), None, TokenType::Access, &paseto.secret).unwrap();
        let claims = get_claim_public(&token, &paseto.public).unwrap();

        // dbg!(&token);
        // dbg!(general_purpose::URL_SAFE_NO_PAD.encode(&paseto.public.as_bytes()));
        // dbg!(claims.iat);

        assert_eq!(claims.sub, Ulid(999).to_string());
        assert_eq!(claims.act, TokenType::Access.string());
        assert_eq!(claims.exp, claims.iat + (3 * 60));
    }

    #[test]
    fn test_gen_old_access_token_and_get_claim_public() {
        let paseto = AsymmetricKeyPair::<V4>::generate().expect("Fail generating keypair");
        let token = gen_token_public(999, (3, 30), Some(1234567890), TokenType::Access, &paseto.secret).unwrap();
        let claims = get_claim_public(&token, &paseto.public).unwrap();

        assert_eq!(claims.sub, Ulid(999).to_string());
        assert_eq!(claims.act, TokenType::Access.string());
        assert_eq!(claims.exp, claims.iat + (3 * 60));
    }

    #[test]
    fn test_gen_new_refresh_token_and_get_claim_public() {
        let paseto = AsymmetricKeyPair::<V4>::generate().expect("Fail generating keypair");
        let token = gen_token_public(999, (3, 30), None, TokenType::Refresh, &paseto.secret).unwrap();
        let claims = get_claim_public(&token, &paseto.public).unwrap();

        assert_eq!(claims.sub, Ulid(999).to_string());
        assert_eq!(claims.act, TokenType::Refresh.string());
        assert_eq!(claims.exp, claims.iat + (30 * 60));
    }

    #[test]
    fn test_gen_old_refresh_token_and_get_claim_public() {
        let paseto = AsymmetricKeyPair::<V4>::generate().expect("Fail generating keypair");
        let token = gen_token_public(999, (3, 30), Some(1234567890), TokenType::Refresh, &paseto.secret).unwrap();
        let claims = get_claim_public(&token, &paseto.public).unwrap();

        assert_eq!(claims.sub, Ulid(999).to_string());
        assert_eq!(claims.act, TokenType::Refresh.string());
        assert_eq!(claims.exp, 1234567890);
    }
}
