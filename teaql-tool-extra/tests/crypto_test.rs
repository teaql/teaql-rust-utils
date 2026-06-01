use teaql_tool_extra::crypto::CryptoTool;

#[test]
fn test_crypto_operations() {
    let tool = CryptoTool::new();
    let key = tool.generate_key();
    assert_eq!(key.len(), 32); // 256 bits

    let data = b"secret message";
    let encrypted = tool.encrypt(data, &key).unwrap();
    assert_ne!(data, encrypted.as_slice());

    let decrypted = tool.decrypt(&encrypted, &key).unwrap();
    assert_eq!(data, decrypted.as_slice());
}
