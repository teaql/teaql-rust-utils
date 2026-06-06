use teaql_tool_std::text::TextTool;

#[test]
fn test_text_manipulation() {
    let tool = TextTool::new();

    assert_eq!(tool.trim("  hello  ").purpose("test"), "hello");
    assert_eq!(tool.lowercase("HELLO").purpose("test"), "hello");
    assert_eq!(tool.uppercase("hello").purpose("test"), "HELLO");
    assert_eq!(tool.capitalize("hello world").purpose("test"), "Hello world");

    assert_eq!(tool.to_snake_case("HelloWorld").purpose("test"), "hello_world");
    assert_eq!(tool.to_camel_case("hello_world").purpose("test"), "helloWorld");
    assert_eq!(tool.to_pascal_case("hello_world").purpose("test"), "HelloWorld");
    assert_eq!(tool.to_kebab_case("helloWorld").purpose("test"), "hello-world");

    assert_eq!(
        tool.normalize_whitespace("  hello \t world \n ").purpose("test"),
        "hello world"
    );

    assert!(tool.contains("hello world", "world").purpose("test"));
    assert!(tool.starts_with("hello world", "hello").purpose("test"));
    assert!(tool.ends_with("hello world", "world").purpose("test"));
}
