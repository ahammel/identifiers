use identifiers_uri::UriIdentifier;
use identifiers_uri::__private::fluent_uri::Uri;

#[derive(UriIdentifier)]
#[allowed_values(typo)]
struct BadId(Uri<String>);

fn main() {}
