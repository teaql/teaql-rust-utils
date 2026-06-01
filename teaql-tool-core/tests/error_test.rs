use teaql_tool_core::{Result, TeaQLToolError};

#[test]
fn test_error_conversion() {
    // We expect to be able to create standard error variants and use Result aliases.
    let err = TeaQLToolError::ParseError("invalid date format".to_string());

    assert_eq!(err.to_string(), "Parse Error: invalid date format");

    let res: Result<i32> = Err(err);
    assert!(res.is_err());
}

#[test]
fn test_invalid_argument_error() {
    let err = TeaQLToolError::InvalidArgument("negative length not allowed".to_string());
    assert_eq!(
        err.to_string(),
        "Invalid Argument: negative length not allowed"
    );
}
