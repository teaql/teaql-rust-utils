use teaql_tool_extra::jwt::JwtTool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Claims {
    sub: String,
    exp: usize,
}

#[test]
fn test_jwt_operations() {
    let tool = JwtTool::new();
    
    let claims = Claims {
        sub: "user123".to_owned(),
        exp: 10000000000,
    };
    
    let token = tool.sign(&claims, "secret").unwrap();
    let verified: Claims = tool.verify(&token, "secret").unwrap();
    
    assert_eq!(verified.sub, "user123");
}
