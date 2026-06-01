use teaql_tool_std::i18n::I18nTool;

#[test]
fn test_i18n_operations() {
    let mut tool = I18nTool::new();
    
    let json_en = r#"{
        "hello": "Hello",
        "messages": {
            "welcome": "Welcome, {name}!"
        }
    }"#;
    
    let json_zh = r#"{
        "hello": "你好",
        "messages": {
            "welcome": "欢迎，{name}！"
        }
    }"#;
    
    tool.load_json("en", json_en).unwrap();
    tool.load_json("zh_CN", json_zh).unwrap();
    
    // Normal translation
    assert_eq!(tool.t("zh_CN", "hello"), "你好");
    assert_eq!(tool.t("en", "hello"), "Hello");
    
    // Nested keys
    assert_eq!(tool.t("zh_CN", "messages.welcome"), "欢迎，{name}！");
    
    // Formatted translation
    assert_eq!(
        tool.tf("zh_CN", "messages.welcome", &[("name", "Alice")]),
        "欢迎，Alice！"
    );
    
    // Fallback to default locale (en)
    assert_eq!(tool.t("fr", "hello"), "Hello");
    
    // Fallback to key itself if not found
    assert_eq!(tool.t("en", "unknown.key"), "unknown.key");
}
