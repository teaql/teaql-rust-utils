use teaql_tool_std::hash::HashTool;

#[test]
fn test_hash_operations() {
    let tool = HashTool::new();

    let sha256 = tool.sha256(b"hello");
    assert_eq!(sha256.len(), 64);
    assert_eq!(sha256, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");

    let sha512 = tool.sha512(b"hello");
    assert_eq!(sha512.len(), 128);

    let blake = tool.blake3(b"hello");
    assert_eq!(blake.len(), 64);

    let crc = tool.crc32(b"hello");
    assert!(crc > 0);
}
