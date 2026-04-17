use identifiers::{BlankError, EmptyError, IntegerIdentifier, StringIdentifier};

#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
struct NonEmptyId(String);

#[derive(StringIdentifier)]
#[allowed_values(non_blank)]
struct NonBlankId(String);

#[derive(StringIdentifier)]
#[allowed_values(all)]
struct AllId(String);

#[derive(IntegerIdentifier)]
#[allowed_values(all)]
struct TestIntId(u64);

// --- non_empty ---

#[test]
fn non_empty_accepts_nonempty_string() {
    assert_eq!(
        NonEmptyId::try_from("hello".to_string()).unwrap().as_str(),
        "hello",
    );
}

#[test]
fn non_empty_rejects_empty_string() {
    assert_eq!(NonEmptyId::try_from(String::new()).unwrap_err(), EmptyError,);
}

#[test]
fn non_empty_accepts_blank_string() {
    assert!(NonEmptyId::try_from("   ".to_string()).is_ok());
}

#[test]
fn non_empty_debug() {
    let id = NonEmptyId::try_from("hello".to_string()).unwrap();
    assert_eq!(format!("{:?}", id), r#"NonEmptyId("hello")"#);
}

// --- non_blank ---

#[test]
fn non_blank_accepts_nonempty_string() {
    assert_eq!(
        NonBlankId::try_from("hello".to_string()).unwrap().as_str(),
        "hello",
    );
}

#[test]
fn non_blank_rejects_empty_string() {
    assert_eq!(NonBlankId::try_from(String::new()).unwrap_err(), BlankError,);
}

#[test]
fn non_blank_rejects_whitespace_only_string() {
    assert_eq!(
        NonBlankId::try_from("   ".to_string()).unwrap_err(),
        BlankError,
    );
}

#[test]
fn non_blank_debug() {
    let id = NonBlankId::try_from("hello".to_string()).unwrap();
    assert_eq!(format!("{:?}", id), r#"NonBlankId("hello")"#);
}

// --- all ---

#[test]
fn all_accepts_nonempty_string() {
    assert_eq!(AllId::from("hello".to_string()).as_str(), "hello");
}

#[test]
fn all_accepts_empty_string() {
    let _ = AllId::from(String::new());
}

#[test]
fn all_accepts_blank_string() {
    let _ = AllId::from("   ".to_string());
}

#[test]
fn all_debug() {
    let id = AllId::from("hello".to_string());
    assert_eq!(format!("{:?}", id), r#"AllId("hello")"#);
}

// --- integer ---

#[test]
fn integer_identifier_zero() {
    assert_eq!(TestIntId::zero().as_u64(), 0);
}

#[test]
fn integer_identifier_roundtrips() {
    assert_eq!(TestIntId::from(42u64).as_u64(), 42);
}

#[test]
fn integer_identifier_ordering() {
    assert!(TestIntId::from(1u64) < TestIntId::from(2u64));
}

#[test]
fn integer_identifier_debug() {
    assert_eq!(format!("{:?}", TestIntId::from(42u64)), "TestIntId(42)");
}

#[test]
fn integer_identifier_from_u8() {
    assert_eq!(TestIntId::from(255u8).as_u64(), 255);
}

#[test]
fn integer_identifier_from_u16() {
    assert_eq!(TestIntId::from(1000u16).as_u64(), 1000);
}

#[test]
fn integer_identifier_from_u32() {
    assert_eq!(TestIntId::from(u32::MAX).as_u64(), u32::MAX as u64);
}

#[test]
fn integer_identifier_try_from_u128_ok() {
    assert_eq!(TestIntId::try_from(42u128).unwrap().as_u64(), 42);
}

#[test]
fn integer_identifier_try_from_u128_overflow() {
    assert!(TestIntId::try_from(u128::MAX).is_err());
}

#[test]
fn integer_identifier_try_from_signed_ok() {
    assert_eq!(TestIntId::try_from(1i8).unwrap().as_u64(), 1);
    assert_eq!(TestIntId::try_from(1i16).unwrap().as_u64(), 1);
    assert_eq!(TestIntId::try_from(1i32).unwrap().as_u64(), 1);
    assert_eq!(TestIntId::try_from(1i64).unwrap().as_u64(), 1);
    assert_eq!(TestIntId::try_from(1i128).unwrap().as_u64(), 1);
}

#[test]
fn integer_identifier_try_from_signed_negative() {
    assert!(TestIntId::try_from(-1i8).is_err());
    assert!(TestIntId::try_from(-1i16).is_err());
    assert!(TestIntId::try_from(-1i32).is_err());
    assert!(TestIntId::try_from(-1i64).is_err());
    assert!(TestIntId::try_from(-1i128).is_err());
}
