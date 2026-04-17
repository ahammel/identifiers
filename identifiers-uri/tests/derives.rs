use fluent_uri::Uri;
use identifiers_uri::UriIdentifier;

#[derive(UriIdentifier)]
struct TestUriId(Uri<String>);

#[test]
fn uri_identifier_roundtrips() {
    let uri = Uri::<String>::parse("https://example.com/foo".to_string()).unwrap();
    assert_eq!(
        TestUriId::try_from(uri).unwrap().as_uri().as_str(),
        "https://example.com/foo"
    );
}

#[test]
fn uri_identifier_debug() {
    let uri = Uri::<String>::parse("https://example.com/foo".to_string()).unwrap();
    assert_eq!(
        format!("{:?}", TestUriId::try_from(uri).unwrap()),
        r#"TestUriId("https://example.com/foo")"#,
    );
}
