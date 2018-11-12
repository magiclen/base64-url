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