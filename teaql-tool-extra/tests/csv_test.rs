use teaql_tool_extra::csv::CsvTool;

#[test]
fn test_csv_operations() {
    let tool = CsvTool::new();
    let data = "name,age\nAlice,30\nBob,25";
    
    let parsed = tool.parse(data).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0][0], "Alice");
    assert_eq!(parsed[1][1], "25");

    let csv_str = tool.generate(&vec![
        vec!["name".to_string(), "age".to_string()],
        vec!["Alice".to_string(), "30".to_string()]
    ]).unwrap();
    
    assert!(csv_str.contains("Alice,30"));
}
