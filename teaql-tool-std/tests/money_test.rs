use iso_currency::Currency;
use rust_decimal_macros::dec;
use teaql_tool_std::money::{Money, MoneyTool};

#[test]
fn test_money_operations() {
    let tool = MoneyTool::new();

    let m1 = tool.of("100.50", "USD").unwrap();
    let m2 = tool.of("50.25", "USD").unwrap();

    assert!(tool.same_currency(&m1, &m2));

    let sum = tool.add(&m1, &m2).unwrap();
    assert_eq!(sum.amount, dec!(150.75));

    let sub = tool.sub(&m1, &m2).unwrap();
    assert_eq!(sub.amount, dec!(50.25));

    let mul = tool.mul(&m1, dec!(2)).unwrap();
    assert_eq!(mul.amount, dec!(201.0));

    let div = tool.div(&m1, dec!(2)).unwrap();
    assert_eq!(div.amount, dec!(50.25));

    let allocated = tool.allocate(&m1, vec![1, 1, 1]).unwrap();
    assert_eq!(allocated.len(), 3);
    assert_eq!(allocated[0].amount, dec!(33.50));
    assert_eq!(allocated[1].amount, dec!(33.50));
    assert_eq!(allocated[2].amount, dec!(33.50));

    let m3 = tool.of("10.125", "USD").unwrap();
    let rounded = tool.round(&m3).unwrap();
    assert_eq!(rounded.amount, dec!(10.12)); // MidpointNearestEven rounds 10.125 to 10.12

    let formatted = tool.format(&rounded);
    assert!(formatted.contains("10.12"));
}
