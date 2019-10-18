use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub(crate) struct TokenRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Serialize)]
pub(crate) struct TokenResponse<'s> {
    pub(crate) user_token: &'s str,
}

#[derive(Deserialize, Default)]
pub(crate) struct LogoutRequest;

#[derive(Serialize)]
pub(crate) struct LogoutResponse;
