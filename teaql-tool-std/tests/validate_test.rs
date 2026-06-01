use teaql_tool_std::validate::ValidateTool;

#[test]
fn test_validation_operations() {
    let tool = ValidateTool::new();

    assert!(tool.email("test@example.com"));
    assert!(!tool.email("invalid-email"));

    assert!(tool.url("https://google.com"));
    assert!(!tool.url("not-a-url"));

    assert!(tool.min_length("hello", 3));
    assert!(!tool.min_length("hi", 3));

    assert!(tool.max_length("hello", 10));
    assert!(!tool.max_length("hello world", 5));
}
