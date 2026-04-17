use identifiers_uri::UriIdentifier;

#[derive(UriIdentifier)]
#[allowed_values(all)]
struct BadId(u64);

fn main() {}
