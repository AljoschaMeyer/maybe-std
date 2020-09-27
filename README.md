# Maybe-std

A rust crate that helps writing libraries that work without [alloc](https://doc.rust-lang.org/alloc/index.html) and/or [std](https://doc.rust-lang.org/std/index.html) but can provide extra functionality if they are available. Writing such libraries involves some namespace-juggling which this crate takes care of. It exports all the items from `core`, `alloc` or `std`, depending on which features are enabled.

## Usage

Depend on this crate by adding it to your `Cargo.toml` file:

```toml
[dependencies]
maybe-std = "0.1.0"
# If the library always requires the `alloc` crate, directly enable the feature:
# maybe-std = { version = "0.1.0", features = [ "alloc" ] }
```

Define the feature flags that control whether your library uses `alloc` and/or `std`, and forward them to the `maybe_std` crate:

```toml
[features]
alloc = [ "maybe_std/alloc" ] # Remove if the library always requires `alloc`
std = [ "maybe_std/std" ]
```

In the crate root, disable the standard library and import this crate - optionally rename it to something short to keep things more readable.

```rust
#![no_std]
extern crate maybe_std as base;

// `base` contains the same items as `core`, `alloc` or `std`, depending on
// the enabled features.
```

In all files, import the prelude:

```rust
use base::prelude::v1::*;
```

When using `std` functionality, gate it with `#[cfg(feature = "std")]`, and when using `alloc` functionality that should be available even without `std`, gate it with `#[cfg(any(feature = "alloc", feature = "std"))]`:

```rust
#[cfg(feature = "std")]
pub const HOME: base::net::Ipv4Addr = base::net::Ipv4Addr::LOCALHOST;

#[cfg(any(feature = "alloc", feature = "std"))]
pub fn empty_string() -> String {
    String::new()
}
```

### Accessing unstable functionality

By default, this crate does not export any [unstable features](https://doc.rust-lang.org/unstable-book/the-unstable-book.html) of `alloc`. These can be enabled via the `unstable` feature flag.
