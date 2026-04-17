use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
#[validate(non_empty)]
struct BadId(u64);

fn main() {}
