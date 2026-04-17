use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
struct BadId(u64);

fn main() {}
