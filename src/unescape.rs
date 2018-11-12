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