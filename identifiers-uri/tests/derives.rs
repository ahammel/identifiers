use identifiers_uri::UriIdentifier;
use identifiers_uri::__private::fluent_uri::Uri;

#[derive(UriIdentifier)]
struct TestUriId(Uri<String>);

#[test]
fn uri_identifier_roundtrips() {
    let uri = Uri::<String>::parse("https://example.com/foo".to_string()).unwrap();
    assert_eq!(
        TestUriId::from(uri).as_uri().as_str(),
        "https://example.com/foo"
    );
}

#[test]
fn uri_identifier_debug() {
    let uri = Uri::<String>::parse("https://example.com/foo".to_string()).unwrap();
    assert_eq!(
        format!("{:?}", TestUriId::from(uri)),
        r#"TestUriId("https://example.com/foo")"#,
    );
}
