use identifiers_uuid::UuidIdentifier;
use identifiers_uuid::__private::uuid::Uuid;

#[derive(UuidIdentifier)]
#[allowed_values(all)]
struct TestUuidId(Uuid);

#[test]
fn uuid_identifier_new_is_nonnil() {
    assert_ne!(TestUuidId::new().as_uuid(), Uuid::nil());
}

#[test]
fn uuid_identifier_new_is_unique() {
    assert_ne!(TestUuidId::new(), TestUuidId::new());
}

#[test]
fn uuid_identifier_roundtrips() {
    let uuid = Uuid::new_v4();
    assert_eq!(TestUuidId::from(uuid).as_uuid(), uuid);
}

#[test]
fn uuid_identifier_debug() {
    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    assert_eq!(
        format!("{:?}", TestUuidId::from(uuid)),
        "TestUuidId(550e8400-e29b-41d4-a716-446655440000)",
    );
}
