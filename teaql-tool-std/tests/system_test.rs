use teaql_tool_std::system::SystemTool;
use std::env;

#[test]
fn test_system_operations() {
    let tool = SystemTool::new();
    
    unsafe { env::set_var("TEAQL_TEST_ENV", "123") };
    
    assert_eq!(tool.env("TEAQL_TEST_ENV").unwrap(), "123");
    assert_eq!(tool.env_or("TEAQL_NON_EXIST", "default"), "default");
    
    assert!(!tool.os().is_empty());
    assert!(!tool.arch().is_empty());
    assert!(tool.current_dir().is_some());
}
