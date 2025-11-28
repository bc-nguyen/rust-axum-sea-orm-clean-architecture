use anyhow::Ok;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}

#[derive(Debug, Clone)]
pub struct JwtHelper {
    secret: String,
}

impl JwtHelper {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate(&self, sub: String) -> anyhow::Result<String> {
        let now = Utc::now();
        let expire: chrono::TimeDelta = Duration::minutes(5);
        let exp = (now + expire).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claim = TokenClaims { exp, iat, sub };

        let value = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(value)
    }

    pub fn validate(&self, token: &str) -> anyhow::Result<String> {
        let result = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(result.claims.sub)
    }
}
