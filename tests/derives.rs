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
