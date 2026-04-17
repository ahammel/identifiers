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

Give each ID its own type. You can do this with plain Rust newtypes:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(s: String) -> Result<Self, EmptyError> {
        if s.is_empty() { return Err(EmptyError); }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str { &self.0 }
}

impl TryFrom<String> for UserId {
    type Error = EmptyError;
    fn try_from(s: String) -> Result<Self, EmptyError> { Self::new(s) }
}
```

That's ~20 lines per type with no guarantee that `new`, `TryFrom`, and
`as_str` stay in sync. Multiply by every identifier in the codebase and the
boilerplate dominates.

This library generates the same code from two lines:

```rust
use identifiers::StringIdentifier;

#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
pub struct UserId(String);

#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
pub struct PostId(String);

#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
pub struct OrgId(String);

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

No runtime overhead.

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

The derive macro always implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`,
and `AsRef<str>`. The `#[allowed_values(...)]` attribute controls what
construction impl is generated.

**`#[allowed_values(all)]`** — derives `From<String>`, accepts any string:

```rust
#[derive(StringIdentifier)]
#[allowed_values(all)]
pub struct Tag(String);

let tag = Tag::from("rust".to_string());
assert_eq!(tag.as_str(), "rust");
```

**`#[allowed_values(non_empty)]`** — derives `TryFrom<String>`, rejects empty strings:

```rust
#[derive(StringIdentifier)]
#[allowed_values(non_empty)]
pub struct UserId(String);

assert!(UserId::try_from("u_abc123".to_string()).is_ok());
assert_eq!(UserId::try_from(String::new()).unwrap_err(), EmptyError);
```

**`#[allowed_values(non_blank)]`** — derives `TryFrom<String>`, rejects empty and
whitespace-only strings:

```rust
#[derive(StringIdentifier)]
#[allowed_values(non_blank)]
pub struct DisplayName(String);

assert!(DisplayName::try_from("Alice".to_string()).is_ok());
assert_eq!(DisplayName::try_from(String::new()).unwrap_err(), BlankError);
assert_eq!(DisplayName::try_from("   ".to_string()).unwrap_err(), BlankError);
```

**No attribute** — no conversion impl is derived; supply your own `From`/`TryFrom`
if desired. The `StringIdentifier` impl is still generated (with infallible
validation).

### Integer identifiers

```rust
use identifiers::IntegerIdentifier;

#[derive(IntegerIdentifier)]
#[allowed_values(all)]
pub struct InvoiceNumber(u64);
```

The derive macro implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`,
`Hash`, `PartialOrd`, and `Ord`. With `#[allowed_values(all)]` it also derives
`From<u64>` and an `IntegerIdentifier` impl.

```rust
let n = InvoiceNumber::from(1042);
assert_eq!(n.as_u64(), 1042);
assert!(InvoiceNumber::from(1) < InvoiceNumber::from(2));

// Zero value (useful as a sentinel or default floor):
let start = InvoiceNumber::zero();
```

Without the attribute, no `From` is derived; supply your own `From`/`TryFrom`
and `IntegerIdentifier` impl.

### UUID identifiers

```toml
[dependencies]
identifiers-uuid = "0.1"
```

```rust
use identifiers_uuid::UuidIdentifier;

#[derive(UuidIdentifier)]
#[allowed_values(all)]
pub struct SessionId(Uuid);
```

With `#[allowed_values(all)]`, also derives `From<Uuid>` and a
`UuidIdentifier` impl. `SessionId::new()` generates a random v4 UUID.

```rust
let id = SessionId::new();
assert_ne!(id, SessionId::new()); // each call produces a unique value

let roundtripped = SessionId::from(id.as_uuid());
assert_eq!(id, roundtripped);
```

### URI identifiers

```toml
[dependencies]
identifiers-uri = "0.1"
```

```rust
use fluent_uri::Uri;
use identifiers_uri::UriIdentifier;

#[derive(UriIdentifier)]
#[allowed_values(all)]
pub struct ResourceUri(Uri<String>);
```

With `#[allowed_values(all)]`, also derives `From<Uri<String>>` and a
`UriIdentifier` impl.

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
#[allowed_values(non_empty)]
pub struct UserId(String);

let mut scores: HashMap<UserId, u32> = HashMap::new();
scores.insert(UserId::try_from("u_1".to_string()).unwrap(), 100);
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
#[allowed_values(non_empty)]
struct AlsoBad(u64);
```

## License

MIT

---

Image via [@_anmonteiro](https://x.com/_anmonteiro/status/1652111152695087104).
