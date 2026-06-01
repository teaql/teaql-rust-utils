use teaql_tool_std::regex::RegexTool;

#[test]
fn test_regex_operations() {
    let tool = RegexTool::new();

    assert!(tool.is_match(r"^\d{4}-\d{2}-\d{2}$", "2023-10-15").unwrap());
    assert!(!tool.is_match(r"^\d{4}-\d{2}-\d{2}$", "2023/10/15").unwrap());

    let f = tool.find(r"\d+", "abc 123 def").unwrap();
    assert_eq!(f.unwrap(), "123");

    let all_f = tool.find_all(r"\d+", "12 34 56").unwrap();
    assert_eq!(all_f, vec!["12", "34", "56"]);

    let rep = tool.replace(r"\s+", "hello   world", " ").unwrap();
    assert_eq!(rep, "hello world");

    let esc = tool.escape(r"hello.world");
    assert_eq!(esc, r"hello\.world");

    assert!(tool.validate(r"^\d+$"));
    assert!(!tool.validate(r"^\d+("));
}
