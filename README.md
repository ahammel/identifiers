# identifiers

<p align="center"><img src="./assets/types-are-a-lie.jpg" alt="String -> String -> String -> String / Types are a lie" width="265" height="360"></p>

Typed identifier wrappers for Rust. Derive newtypes over `String`, `u64`,
`Uuid`, and URI that are mutually incompatible at the type level, eliminating
an entire class of argument-order bugs.

## The problem

APIs that take several IDs of the same underlying type are easy to misuse:

```rust
fn transfer_post(
    from_user: &str,
    to_user: &str,
    post_id: &str,
    org_id: &str,
) -> Result<(), Error> {
    // ...
}

// Compiles and runs. The bug ships.
transfer_post(&post_id, &org_id, &from_user, &to_user)?;
```

The compiler sees four `&str` parameters and happily accepts any permutation.
Tests may not catch it. The wrong record gets modified in production.

## The solution

Give each ID its own type. The compiler rejects wrong-order calls:

```rust
use identifiers::{StringIdentifier, IntegerIdentifier};

#[derive(StringIdentifier)] pub struct UserId(String);
#[derive(StringIdentifier)] pub struct PostId(String);
#[derive(StringIdentifier)] pub struct OrgId(String);

fn transfer_post(
    from_user: &UserId,
    to_user: &UserId,
    post_id: &PostId,
    org_id: &OrgId,
) -> Result<(), Error> {
    // ...
}

// error[E0308]: mismatched types — caught at compile time.
transfer_post(&post_id, &org_id, &from_user, &to_user)?;
```

## Installation

```toml
[dependencies]
identifiers = "0.1"

# Optional — only if you need UUID or URI identifiers:
identifiers-uuid = "0.1"
identifiers-uri  = "0.1"
```

## Defining identifier types

### String identifiers

```rust
use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
pub struct UserId(String);

#[derive(StringIdentifier)]
pub struct PostId(String);
```

The derive macro implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, and
`From<String>` in addition to `StringIdentifier` itself.

```rust
let id = UserId::from("u_abc123".to_string());
assert_eq!(id.as_str(), "u_abc123");

// Debug output includes the type name:
// UserId("u_abc123")
```

### Integer identifiers

```rust
use identifiers::IntegerIdentifier;

#[derive(IntegerIdentifier)]
pub struct InvoiceNumber(u64);
```

Adds `Copy`, `Ord`, `PartialOrd`, and `From<u64>` on top of the common
set, so integer identifiers can be used as map keys and sorted naturally.

```rust
let n = InvoiceNumber::from(1042);
assert_eq!(n.as_u64(), 1042);
assert!(InvoiceNumber::from(1) < InvoiceNumber::from(2));

// Zero value (useful as a sentinel or default floor):
let start = InvoiceNumber::zero();
```

### UUID identifiers

```toml
[dependencies]
identifiers-uuid = "0.1"
```

```rust
use identifiers_uuid::UuidIdentifier;

#[derive(UuidIdentifier)]
pub struct SessionId(Uuid);
```

Adds `Copy` and `From<Uuid>`. `SessionId::new()` generates a random v4 UUID.

```rust
let id = SessionId::new();
assert_ne!(id, SessionId::new()); // each call produces a unique value

let uuid = id.as_uuid();
let roundtripped = SessionId::from(uuid);
assert_eq!(id, roundtripped);
```

### URI identifiers

```toml
[dependencies]
identifiers-uri = "0.1"
```

```rust
use identifiers_uri::UriIdentifier;
use fluent_uri::Uri;

#[derive(UriIdentifier)]
pub struct ResourceUri(Uri<String>);
```

```rust
let uri = Uri::<String>::parse("https://example.com/resources/42".to_string()).unwrap();
let id = ResourceUri::from(uri);
assert_eq!(id.as_uri().as_str(), "https://example.com/resources/42");
```

## Using identifiers as map keys

All identifier types implement `Hash` and `Eq`, so they work directly as
`HashMap` and `HashSet` keys.

```rust
use std::collections::HashMap;
use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
pub struct UserId(String);

let mut scores: HashMap<UserId, u32> = HashMap::new();
scores.insert(UserId::from("u_1".to_string()), 100);
```

## Type constraints

Each derive macro only accepts a single-field tuple struct wrapping the
appropriate inner type. Annotating the wrong type or a non-newtype struct is a
compile error:

```rust
// error: expected a newtype struct with exactly one unnamed field
#[derive(StringIdentifier)]
struct Bad { id: String }

// error[E0308]: mismatched types
#[derive(StringIdentifier)]
struct AlsoBad(u64);
```

## License

MIT

---

Image via [@_anmonteiro](https://x.com/_anmonteiro/status/1652111152695087104).
