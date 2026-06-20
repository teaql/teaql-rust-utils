use crate::macros::define_context_facade;

use teaql_tool_extra::jwt::JwtTool;
use serde::{Deserialize, Serialize};
use teaql_tool_core::Result;

define_context_facade!("extra", jwt, ContextJwtExt, ContextJwtFacade);

#[cfg(feature = "extra")]
impl<'a> ContextJwtFacade<'a> {
    /// Sign claims into a JWT token using the given secret.
    pub fn sign<T: Serialize>(&self, claims: &T, secret: &str) -> Result<String> {
        JwtTool::new().sign(claims, secret)
    }

    /// Verify and decode a JWT token using the given secret.
    pub fn verify<T: for<'de> Deserialize<'de>>(&self, token: &str, secret: &str) -> Result<T> {
        JwtTool::new().verify(token, secret)
    }
}
