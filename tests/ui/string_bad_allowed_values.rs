use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
#[allowed_values(typo)]
struct BadId(String);

fn main() {}
