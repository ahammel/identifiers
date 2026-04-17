use identifiers_uuid::UuidIdentifier;
use identifiers_uuid::__private::uuid::Uuid;

#[derive(UuidIdentifier)]
#[allowed_values(typo)]
struct BadId(Uuid);

fn main() {}
