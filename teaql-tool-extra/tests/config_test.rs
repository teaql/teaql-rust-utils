use teaql_tool_extra::config::ConfigTool;
use std::env;

#[test]
fn test_config_operations() {
    let tool = ConfigTool::new();
    
    tool.load_env().ok();
    
    unsafe { env::set_var("TEAQL_CFG", "abcd") };
    assert_eq!(tool.get_env("TEAQL_CFG").unwrap(), "abcd");
}
