use teaql_tool_std::filter::FilterTool;

#[test]
fn test_filter_operations() {
    let tool = FilterTool::new();
    let words = vec!["badword", "ugly"];
    let trie = tool.build_trie(&words);
    
    let text1 = "this is a badword!";
    let text2 = "this is very nice!";
    
    assert!(tool.contains_sensitive(text1, &trie));
    assert!(!tool.contains_sensitive(text2, &trie));
    
    let replaced = tool.replace_sensitive(text1, &trie, "***");
    assert_eq!(replaced, "this is a ***!");
}
