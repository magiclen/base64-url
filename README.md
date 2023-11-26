Base64 URL
====================

[![CI](https://github.com/magiclen/base64-url/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/base64-url/actions/workflows/ci.yml)

Base64 encode, decode, escape and unescape for URL applications.

## Examples

Encode data to a Base64-URL string.

```rust
assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::encode("Hello, world!"));
```

Decode a Base64-URL string to data.

```rust
assert_eq!(b"Hello, world!", base64_url::decode("SGVsbG8sIHdvcmxkIQ").unwrap().as_slice());
```

Escape a Base64 string to a Base64-URL string. The conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.

```rust
assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::escape("SGVsbG8sIHdvcmxkIQ=="));
```

Unescape a Base64-URL string to a Base64-URL string. The conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.

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

## Crates.io

https://crates.io/crates/base64-url

## Documentation

https://docs.rs/base64-url

## License

[MIT](LICENSE)