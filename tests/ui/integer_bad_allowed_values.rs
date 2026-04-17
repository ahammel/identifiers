use identifiers::IntegerIdentifier;

#[derive(IntegerIdentifier)]
#[allowed_values(typo)]
struct BadId(u64);

fn main() {}
