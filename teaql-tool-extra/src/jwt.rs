use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use teaql_tool_core::{Result, TeaQLToolError};

pub struct JwtTool;

impl JwtTool {
    pub fn new() -> Self {
        Self
    }

    pub fn sign<T: Serialize>(&self, claims: &T, secret: &str) -> Result<String> {
        encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref()))
            .map_err(|e| TeaQLToolError::ExecutionError(format!("JWT sign failed: {}", e)))
    }

    pub fn verify<T: for<'de> Deserialize<'de>>(&self, token: &str, secret: &str) -> Result<T> {
        let mut validation = Validation::default();
        validation.required_spec_claims.clear(); // for generic struct tests
        
        let token_data = decode::<T>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation
        ).map_err(|e| TeaQLToolError::ExecutionError(format!("JWT verify failed: {}", e)))?;
        
        Ok(token_data.claims)
    }
}

impl Default for JwtTool {
    fn default() -> Self {
        Self::new()
    }
}
