use teaql_tool::T;

#[test]
fn test_facade_standard_tools() {
    // Time Tool
    let now = T::time().now();
    assert!(now.timestamp() > 0);

    // Id Tool
    let uuid = T::id().uuid();
    assert_eq!(uuid.len(), 36);

    // Codec Tool
    let b64 = T::codec().base64_encode("facade");
    assert_eq!(b64, "ZmFjYWRl");

    // Text Tool
    let text = T::text().trim(" hello ");
    assert_eq!(text, "hello");

    // Json Tool
    let parsed = T::json().parse(r#"{"test": 1}"#).unwrap();
    assert!(T::json().has(&parsed, "/test"));

    // Decimal Tool
    let dec = T::decimal().of("10.5").unwrap();
    assert_eq!(T::decimal().add(dec, dec).to_string(), "21.0");

    // Money Tool
    let money = T::money().of("100", "USD").unwrap();
    assert_eq!(T::money().format(&money), "100 USD");
}

#[test]
#[cfg(feature = "extra")]
fn test_facade_extra_tools() {
    // Random Tool
    let num = T::random().int(1, 10);
    assert!(num >= 1 && num <= 10);

    // Crypto Tool
    let key = T::crypto().generate_key();
    assert_eq!(key.len(), 32);

    // Csv Tool
    let csv = T::csv().parse("a,b\n1,2").unwrap();
    assert_eq!(csv.len(), 1); // Header "a,b" is skipped by default in csv crate
    assert_eq!(csv[0][0], "1");
}
