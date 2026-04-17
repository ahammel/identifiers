use identifiers::{BlankError, EmptyError, IntegerIdentifier, StringIdentifier};

#[derive(StringIdentifier)]
#[validate(non_empty)]
struct NonEmptyId(String);

#[derive(StringIdentifier)]
#[validate(non_blank)]
struct NonBlankId(String);

#[derive(StringIdentifier)]
#[validate(any)]
struct AnyId(String);

#[derive(IntegerIdentifier)]
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

// --- any ---

#[test]
fn any_accepts_nonempty_string() {
    assert_eq!(
        AnyId::try_from("hello".to_string()).unwrap().as_str(),
        "hello",
    );
}

#[test]
fn any_accepts_empty_string() {
    assert!(AnyId::try_from(String::new()).is_ok());
}

#[test]
fn any_accepts_blank_string() {
    assert!(AnyId::try_from("   ".to_string()).is_ok());
}

#[test]
fn any_debug() {
    let id = AnyId::try_from("hello".to_string()).unwrap();
    assert_eq!(format!("{:?}", id), r#"AnyId("hello")"#);
}

// --- integer ---

#[test]
fn integer_identifier_zero() {
    assert_eq!(TestIntId::zero().as_u64(), 0);
}

#[test]
fn integer_identifier_roundtrips() {
    assert_eq!(TestIntId::try_from(42).unwrap().as_u64(), 42);
}

#[test]
fn integer_identifier_ordering() {
    assert!(TestIntId::try_from(1).unwrap() < TestIntId::try_from(2).unwrap());
}

#[test]
fn integer_identifier_debug() {
    assert_eq!(
        format!("{:?}", TestIntId::try_from(42).unwrap()),
        "TestIntId(42)"
    );
}
