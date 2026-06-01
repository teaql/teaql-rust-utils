use teaql_tool_extra::cache::CacheTool;

#[test]
fn test_cache_operations() {
    let tool = CacheTool::new();
    
    tool.put("key1", "val1");
    assert_eq!(tool.get("key1").unwrap(), "val1");
    assert!(tool.get("key2").is_none());
}
