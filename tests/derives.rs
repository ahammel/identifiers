use identifiers::{IntegerIdentifier, StringIdentifier};

#[derive(StringIdentifier)]
struct TestStringId(String);

#[derive(IntegerIdentifier)]
struct TestIntId(u64);

#[test]
fn string_identifier_roundtrips() {
    let s = "hello".to_string();
    assert_eq!(TestStringId::from(s.clone()).as_str(), s);
}

#[test]
fn string_identifier_debug() {
    assert_eq!(
        format!("{:?}", TestStringId::from("hello".to_string())),
        r#"TestStringId("hello")"#,
    );
}

#[test]
fn integer_identifier_zero() {
    assert_eq!(TestIntId::zero().as_u64(), 0);
}

#[test]
fn integer_identifier_roundtrips() {
    assert_eq!(TestIntId::from(42).as_u64(), 42);
}

#[test]
fn integer_identifier_ordering() {
    assert!(TestIntId::from(1) < TestIntId::from(2));
}

#[test]
fn integer_identifier_debug() {
    assert_eq!(format!("{:?}", TestIntId::from(42)), "TestIntId(42)");
}
