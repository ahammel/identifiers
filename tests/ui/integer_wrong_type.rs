use identifiers::IntegerIdentifier;

#[derive(IntegerIdentifier)]
#[allowed_values(all)]
struct BadId(String);

fn main() {}
