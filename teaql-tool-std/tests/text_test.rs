use teaql_tool_std::text::TextTool;

#[test]
fn test_text_manipulation() {
    let tool = TextTool::new();

    assert_eq!(tool.trim("  hello  "), "hello");
    assert_eq!(tool.lowercase("HELLO"), "hello");
    assert_eq!(tool.uppercase("hello"), "HELLO");
    assert_eq!(tool.capitalize("hello world"), "Hello world");

    assert_eq!(tool.to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(tool.to_camel_case("hello_world"), "helloWorld");
    assert_eq!(tool.to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(tool.to_kebab_case("helloWorld"), "hello-world");

    assert_eq!(
        tool.normalize_whitespace("  hello \t world \n "),
        "hello world"
    );

    assert!(tool.contains("hello world", "world"));
    assert!(tool.starts_with("hello world", "hello"));
    assert!(tool.ends_with("hello world", "world"));
}
