//! # Base64 URL
//!
//! Base64 encode, decode, escape and unescape for URL applications.
//!
//! ## Examples
//!
//! Encode data to a Base64-URL string.
//!
//! ```
//! extern crate base64_url;
//!
//! assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::encode("Hello, world!"));
//! ```
//!
//! Decode a Base64-URL string to data.
//!
//! ```
//! extern crate base64_url;
//!
//! assert_eq!("Hello, world!".as_bytes().to_vec(), base64_url::decode("SGVsbG8sIHdvcmxkIQ").unwrap());
//! ```
//!
//! Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
//!
//! ```
//! extern crate base64_url;
//!
//! assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::unsafe_escape("SGVsbG8sIHdvcmxkIQ=="));
//! ```
//!
//! Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
//!
//! ```
//! extern crate base64_url;
//!
//! assert_eq!("SGVsbG8sIHdvcmxkIQ==", base64_url::unsafe_unescape("SGVsbG8sIHdvcmxkIQ"));
//! ```
//!
//! Besides, in order to reduce the copy times of strings, you can also use `encode_and_push_to_string`, `decode_and_push_to_vec`, `unsafe_escape_owned` and `unsafe_unescape_owned` associated functions to use the same memory space.

pub extern crate base64;

/// Encode data to a Base64-URL string.
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::encode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Encode data to a Base64-URL string into a slice.
pub fn encode_and_store_to_slice<T: ?Sized + AsRef<[u8]>>(input: &T, output: &mut [u8]) -> usize {
    base64::encode_config_slice(input, base64::URL_SAFE_NO_PAD, output)
}

/// Encode data to a Base64-URL string and directly store into a String instance by concatenating them. It is usually for generating a URL.
pub fn encode_and_push_to_string<T: ?Sized + AsRef<[u8]>>(input: &T, output: String) -> String {
    let bytes = input.as_ref();

    let mut buffer = output.into_bytes();

    let current_len = buffer.len();

    let base64_len = (bytes.len() * 4 + 2) / 3;

    let min_capacity = current_len + base64_len;

    let capacity = buffer.capacity();

    if capacity < min_capacity {
        buffer.reserve(min_capacity - capacity);
    }

    unsafe {
        buffer.set_len(min_capacity);
    }

    let base64_len = encode_and_store_to_slice(bytes, &mut buffer[current_len..min_capacity]);

    unsafe {
        buffer.set_len(current_len + base64_len);
    }

    unsafe {
        String::from_utf8_unchecked(buffer)
    }
}

/// Decode a Base64-URL string to data.
pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Decode a Base64-URL string to data into a slice.
pub fn decode_and_store_to_slice<T: ?Sized + AsRef<[u8]>>(input: &T, output: &mut [u8]) -> Result<usize, base64::DecodeError> {
    base64::decode_config_slice(input, base64::URL_SAFE_NO_PAD, output)
}

/// Decode a Base64-URL string to data and directly store into a Vec instance by concatenating them.
pub fn decode_and_push_to_vec<T: ?Sized + AsRef<[u8]>>(input: &T, mut output: Vec<u8>) -> Result<Vec<u8>, base64::DecodeError> {
    let bytes = input.as_ref();

    let current_len = output.len();

    let original_max_len = {
        let len = bytes.len();
        (len + 3 / 4) * 3
    };

    let min_capacity = current_len + original_max_len;

    let capacity = output.capacity();

    if capacity < min_capacity {
        output.reserve(min_capacity - capacity);
    }

    unsafe {
        output.set_len(min_capacity);
    }

    let original_len = decode_and_store_to_slice(bytes, &mut output[current_len..min_capacity])?;

    unsafe {
        output.set_len(current_len + original_len);
    }

    Ok(output)
}

/// Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
pub fn unsafe_escape(base64_str: &str) -> String {
    let mut result = Vec::with_capacity(base64_str.len());

    for &n in base64_str.as_bytes() {
        if n == 43 {
            result.push(45);
        } else if n == 47 {
            result.push(95);
        } else if n == 61 {
            break;
        } else {
            result.push(n);
        }
    }

    unsafe {
        String::from_utf8_unchecked(result)
    }
}

/// Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
pub fn unsafe_escape_owned(base64_str: String) -> String {
    let mut result = base64_str.into_bytes();

    let mut len = result.len();

    for (index, n) in result.iter_mut().enumerate() {
        match *n {
            43 => *n = 45,
            47 => *n = 95,
            61 => { len = index; }
            _ => ()
        }
    }

    unsafe {
        result.set_len(len);
    }

    unsafe {
        String::from_utf8_unchecked(result)
    }
}

/// Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
pub fn unsafe_unescape(base64_str: &str) -> String {
    let len = base64_str.len();

    let mut result = Vec::with_capacity(((len * 4 / 3) + 3) / 4 * 4);

    for &n in base64_str.as_bytes() {
        if n == 45 {
            result.push(43);
        } else if n == 95 {
            result.push(47);
        } else {
            result.push(n);
        }
    }

    let padding = len % 4;

    if padding > 0 {
        for _ in padding..4 {
            result.push(61);
        }
    }

    unsafe {
        String::from_utf8_unchecked(result)
    }
}

/// Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
pub fn unsafe_unescape_owned(base64_str: String) -> String {
    let mut result = base64_str.into_bytes();

    for n in result.iter_mut() {
        match *n {
            45 => *n = 43,
            95 => *n = 47,
            _ => ()
        }
    }

    let padding = result.len() % 4;

    if padding > 0 {
        for _ in padding..4 {
            result.push(61);
        }
    }

    unsafe {
        String::from_utf8_unchecked(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", encode("https://magiclen.org"));
    }

    #[test]
    fn test_encode_and_push_to_string() {
        let url = "https://magiclen.org/".to_string();

        assert_eq!("https://magiclen.org/YXJ0aWNsZXM", encode_and_push_to_string("articles", url));
    }

    #[test]
    fn test_decode() {
        assert_eq!("https://magiclen.org".as_bytes().to_vec(), decode("aHR0cHM6Ly9tYWdpY2xlbi5vcmc").unwrap());
    }

    #[test]
    fn test_decode_and_push_to_vec() {
        let url = "https://magiclen.org/".as_bytes().to_vec();

        assert_eq!("https://magiclen.org/articles".as_bytes().to_vec(), decode_and_push_to_vec("YXJ0aWNsZXM", url).unwrap());
    }

    #[test]
    fn test_unsafe_escape() {
        assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", unsafe_escape(&base64::encode("https://magiclen.org")));
    }

    #[test]
    fn test_unsafe_escape_owned() {
        assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", unsafe_escape_owned(base64::encode("https://magiclen.org")));
    }

    #[test]
    fn test_unsafe_unescape() {
        assert_eq!("https://magiclen.org".as_bytes().to_vec(), base64::decode(&unsafe_unescape("aHR0cHM6Ly9tYWdpY2xlbi5vcmc")).unwrap());
    }

    #[test]
    fn test_unsafe_unescape_owned() {
        assert_eq!("https://magiclen.org".as_bytes().to_vec(), base64::decode(&unsafe_unescape_owned("aHR0cHM6Ly9tYWdpY2xlbi5vcmc".to_string())).unwrap());
    }
}
