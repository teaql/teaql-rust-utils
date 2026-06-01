use serde_json::json;
use teaql_tool_std::json::JsonTool;

#[test]
fn test_json_operations() {
    let tool = JsonTool::new();

    // parse
    let v = tool.parse(r#"{"a": 1}"#).unwrap();
    assert_eq!(v["a"], 1);

    // stringify
    let s = tool.stringify(&v).unwrap();
    assert_eq!(s, r#"{"a":1}"#);

    // get
    let val = tool.get(&v, "/a").unwrap();
    assert_eq!(val, &json!(1));

    // set
    let mut v2 = v.clone();
    tool.set(&mut v2, "/b", json!(2)).unwrap();
    assert_eq!(v2["b"], 2);

    // remove
    tool.remove(&mut v2, "/a").unwrap();
    assert!(!tool.has(&v2, "/a"));
    assert!(tool.has(&v2, "/b"));

    // merge
    let v3 = tool.merge(&v, &json!({"b": 2}));
    assert_eq!(v3["a"], 1);
    assert_eq!(v3["b"], 2);

    // patch
    let diff = tool.diff(&v, &v3);
    assert!(!diff.is_empty());

    let mut v4 = v.clone();
    tool.patch(&mut v4, &diff).unwrap();
    assert_eq!(v4, v3);
}
