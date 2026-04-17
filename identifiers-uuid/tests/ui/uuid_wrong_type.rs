use identifiers_uuid::UuidIdentifier;

#[derive(UuidIdentifier)]
#[allowed_values(all)]
struct BadId(String);

fn main() {}
