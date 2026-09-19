use teaql_tool_std::id::IdTool;

#[test]
fn test_id_generation() {
    let tool = IdTool::new();

    // Test UUID v4
    let uuid_val = tool.uuid();
    assert_eq!(uuid_val.len(), 36);

    // Test UUID v7
    let uuid7_val = tool.uuid_v7();
    assert_eq!(uuid7_val.len(), 36);

    // Test ULID
    let ulid_val = tool.ulid();
    assert_eq!(ulid_val.len(), 26);

    // Test Nanoid
    let nanoid_val = tool.nanoid();
    assert!(!nanoid_val.is_empty());

    // Test with prefix
    let prefixed = tool.with_prefix("usr");
    assert!(prefixed.starts_with("usr_"));
}
