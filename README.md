Base64 URL
====================

[![Build Status](https://travis-ci.org/magiclen/base64-url.svg?branch=master)](https://travis-ci.org/magiclen/base64-url)
[![Build status](https://ci.appveyor.com/api/projects/status/y24btypttrjtnj6h/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/base64-url/branch/master)

Base64 encode, decode, escape and unescape for URL applications.

## Examples

Encode data to a Base64-URL string.

```rust
extern crate base64_url;

assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::encode("Hello, world!"));
```

Decode a Base64-URL string to data.

```rust
extern crate base64_url;

assert_eq!("Hello, world!".as_bytes().to_vec(), base64_url::decode("SGVsbG8sIHdvcmxkIQ").unwrap());
```

Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.

```rust
extern crate base64_url;

assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::unsafe_escape("SGVsbG8sIHdvcmxkIQ=="));
```

Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.

```rust
extern crate base64_url;

assert_eq!("SGVsbG8sIHdvcmxkIQ==", base64_url::unsafe_unescape("SGVsbG8sIHdvcmxkIQ"));
```

Beside, in order to reduce the copy times of strings, you can also use `encode_and_push_to_string`, `decode_and_push_to_vec`, `unsafe_escape_owned` and `unsafe_unescape_owned` associated functions to use the same memory space.

## Crates.io

https://crates.io/crates/base64-url

## Documentation

https://docs.rs/base64-url

## License

[MIT](LICENSE)