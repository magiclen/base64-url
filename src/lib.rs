/*!
# Base64 URL

Base64 encode, decode, escape and unescape for URL applications.

The `encode*` functions use the URL-safe alphabet (`-` and `_`) without `=` padding, and the `decode*` functions require this format.

## Examples

Encode data to a Base64-URL string.

```rust
assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::encode("Hello, world!"));
```

Decode a Base64-URL string to data.

```rust
assert_eq!(b"Hello, world!", base64_url::decode("SGVsbG8sIHdvcmxkIQ").unwrap().as_slice());
```

Escape a Base64 string to a Base64-URL string.
This function does not validate the input, so make sure it is valid Base64 first.

```rust
assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::escape("SGVsbG8sIHdvcmxkIQ=="));
```

Unescape a Base64-URL string to a Base64 string.
This function does not validate the input, so make sure it is valid Base64-URL first.

```rust
assert_eq!("SGVsbG8sIHdvcmxkIQ==", base64_url::unescape("SGVsbG8sIHdvcmxkIQ"));
```

Besides, you can also use other `encode_*`, `decode_*`, `escape_*`, `unescape_*` associated functions to deal with more specific cases. For example,

```rust
let hash = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
let mut url = String::from("https://example.com/?hash=");

assert_eq!("AQIDBAUGBwgJ", base64_url::encode_to_string(hash, &mut url));
assert_eq!("https://example.com/?hash=AQIDBAUGBwgJ", url);
```

The `encode_to_string`, `encode_to_vec`, and `decode_to_vec` functions append to the output and return only the new data.
The `*_to_slice` functions write from the start of the output slice and return only the written part.
The `decode_to_slice` function may write part of the decoded data before returning an error.

## Std

This crate is `no_std` compatible by default and only requires `alloc`.
Enable the `std` feature to pass `std` support through to the `base64` dependency.

```toml
[dependencies.base64-url]
version = "*"
features = ["std"]
```

## Migrating from 3.x to 4.0

Version 4.0 requires Rust 1.71 or later and uses `base64` 0.23 instead of 0.22.
This crate re-exports `base64`, and its decode functions return error types from that crate.

If you match `DecodeError::InvalidLastSymbol`, change `InvalidLastSymbol(offset, symbol)` to `InvalidLastSymbol { offset, symbol, .. }`.
The new `symbol_value` field contains the decoded six-bit value.
The `Display` output for this variant has changed, and the `Debug` output for `DecodeError` now uses `Display`.
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub extern crate base64;

mod decode;
mod encode;
mod escape;
mod unescape;

pub use decode::*;
pub use encode::*;
pub use escape::*;
pub use unescape::*;
