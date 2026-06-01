use teaql_tool_std::color::ColorTool;

#[test]
fn test_color_operations() {
    let tool = ColorTool::new();
    
    assert_eq!(tool.alice_blue(), "#F0F8FF");
    assert_eq!(tool.black(), "#000000");
    assert_eq!(tool.white(), "#FFFFFF");
    assert_eq!(tool.red(), "#FF0000");
}
