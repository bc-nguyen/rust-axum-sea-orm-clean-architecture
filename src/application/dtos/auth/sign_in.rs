use serde::Serialize;

#[derive(Serialize)]
pub struct ResSignInDto {
    pub access_token: String,
}
