use teaql_tool_extra::random::RandomTool;

#[test]
fn test_random_operations() {
    let tool = RandomTool::new();

    let num = tool.int(1, 10);
    assert!(num >= 1 && num <= 10);

    let float = tool.float(0.0, 1.0);
    assert!(float >= 0.0 && float <= 1.0);

    let bool_val = tool.boolean();
    // Just asserting it compiles and returns a bool
    assert!(bool_val || !bool_val);
}
