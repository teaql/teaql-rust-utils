use teaql_tool_std::id::IdTool;

#[test]
fn test_id_generation() {
    let tool = IdTool::new();

    // Test UUID v4
    let uuid_val = tool.uuid().purpose("test");
    assert_eq!(uuid_val.len(), 36);

    // Test UUID v7
    let uuid7_val = tool.uuid_v7().purpose("test");
    assert_eq!(uuid7_val.len(), 36);

    // Test ULID
    let ulid_val = tool.ulid().purpose("test");
    assert_eq!(ulid_val.len(), 26);

    // Test Nanoid
    let nanoid_val = tool.nanoid().purpose("test");
    assert!(!nanoid_val.is_empty());

    // Test with prefix
    let prefixed = tool.with_prefix("usr").purpose("test");
    assert!(prefixed.starts_with("usr_"));
}
