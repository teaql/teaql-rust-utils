use teaql_tool_std::daterange::DateRangeTool;

#[test]
fn test_daterange_operations() {
    let tool = DateRangeTool::new();
    
    let today = tool.today();
    assert!(today.start <= today.end);

    let yesterday = tool.yesterday();
    assert!(yesterday.start < today.start);

    let next_3_days = tool.next_n_days(3).unwrap();
    assert!(next_3_days.start <= next_3_days.end);
    
    let this_hour = tool.this_hour();
    assert!(this_hour.start <= this_hour.end);
}
