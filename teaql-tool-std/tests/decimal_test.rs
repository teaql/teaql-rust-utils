use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use teaql_tool_std::decimal::DecimalTool;

#[test]
fn test_decimal_operations() {
    let tool = DecimalTool::new();

    let a = tool.of("10.5").unwrap().purpose("test");
    let b = tool.of("2.0").unwrap().purpose("test");

    assert_eq!(tool.add(a, b).purpose("test"), dec!(12.5));
    assert_eq!(tool.sub(a, b).purpose("test"), dec!(8.5));
    assert_eq!(tool.mul(a, b).purpose("test"), dec!(21.0));
    assert_eq!(tool.div(a, b).unwrap().purpose("test"), dec!(5.25));

    let zero = tool.zero().purpose("test");
    assert_eq!(zero, dec!(0.0));

    let one = tool.one().purpose("test");
    assert_eq!(one, dec!(1.0));

    let c = tool.of("10.456").unwrap().purpose("test");
    assert_eq!(tool.round(c, 2).purpose("test"), dec!(10.46));
    assert_eq!(tool.ceil(c).purpose("test"), dec!(11));
    assert_eq!(tool.floor(c).purpose("test"), dec!(10));

    assert_eq!(tool.percent(dec!(200), dec!(15)).purpose("test"), dec!(30)); // 15% of 200
}
