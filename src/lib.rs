/*!
# Base64 URL

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

Besides, you can also use other `encode_*`, `decode_*`, `unsafe_escape_*`, `unsafe_unescape_*` associated functions to deal with more specific cases. For example,

```rust
extern crate base64_url;

let hash = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
let url = String::from("https://example.com/?hash=");

assert_eq!("https://example.com/?hash=AQIDBAUGBwgJ", base64_url::encode_and_push_to_string(hash, url));
```
*/

#![no_std]

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
