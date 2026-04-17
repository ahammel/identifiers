use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
struct NonEmptyPubField(pub String);

fn main() {}
