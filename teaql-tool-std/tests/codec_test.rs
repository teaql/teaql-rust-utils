use teaql_tool_std::codec::CodecTool;

#[test]
fn test_codec_operations() {
    let tool = CodecTool::new();

    // base64
    let b64 = tool.base64_encode("hello");
    assert_eq!(b64, "aGVsbG8=");
    let decoded = tool.base64_decode(&b64).unwrap();
    assert_eq!(decoded, b"hello");

    // hex
    let hx = tool.hex_encode("hello");
    assert_eq!(hx, "68656c6c6f");
    let hex_dec = tool.hex_decode(&hx).unwrap();
    assert_eq!(hex_dec, b"hello");

    // url
    let url_enc = tool.url_encode("hello world/");
    assert_eq!(url_enc, "hello%20world%2F");
    let url_dec = tool.url_decode(&url_enc).unwrap();
    assert_eq!(url_dec, "hello world/");

    // html
    let html_enc = tool.html_escape("<script>alert('1')</script>");
    assert_eq!(html_enc, "&lt;script&gt;alert('1')&lt;/script&gt;");
    let html_dec = tool.html_unescape(&html_enc);
    assert_eq!(html_dec, "<script>alert('1')</script>");
}
