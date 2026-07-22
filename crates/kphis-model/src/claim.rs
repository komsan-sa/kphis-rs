use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub act: String,
    pub iat: u64,
    pub exp: u64,
}
