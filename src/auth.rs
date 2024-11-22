use base64::{engine::general_purpose::STANDARD, Engine};
use core::fmt;

#[derive(Debug, Clone)]
pub struct AuthString(String);

#[derive(Debug, Clone)]
pub struct InvalidAuthStringErr;

impl AuthString {
    pub fn new(auth_str: &str) -> Result<Self, InvalidAuthStringErr> {
        let parts = auth_str.split(':').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(InvalidAuthStringErr);
        }

        Ok(Self(auth_str.to_string()))
    }
}

pub fn parse_auth_string_arg(arg: &str) -> Result<AuthString, &'static str> {
    AuthString::new(arg).map_err(|_| "Auth string is invalid")
}

impl fmt::Display for AuthString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Basic {}", STANDARD.encode(&self.0))
    }
}
