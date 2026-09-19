use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use teaql_tool_core::{MustAuditAs, Result, TeaQLToolError};

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

#[test]
fn audit_action_is_deferred_until_audit_as() {
    let executed = Arc::new(AtomicBool::new(false));
    let marker = Arc::clone(&executed);
    let pending = MustAuditAs::new(move |description| {
        assert_eq!(description, "write report");
        marker.store(true, Ordering::SeqCst);
        42
    });

    assert!(!executed.load(Ordering::SeqCst));
    assert_eq!(pending.audit_as("write report"), 42);
    assert!(executed.load(Ordering::SeqCst));
}

#[test]
fn dropped_audit_action_is_not_executed() {
    let executed = Arc::new(AtomicBool::new(false));
    let marker = Arc::clone(&executed);

    drop(MustAuditAs::new(move |_description| {
        marker.store(true, Ordering::SeqCst);
    }));

    assert!(!executed.load(Ordering::SeqCst));
}
