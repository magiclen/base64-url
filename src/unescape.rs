/// Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
pub fn unsafe_unescape<S: Into<String>>(base64_url: S) -> String {
    let mut result = base64_url.into().into_bytes();

    for n in result.iter_mut() {
        match *n {
            45 => *n = 43,
            95 => *n = 47,
            _ => (),
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

#[deprecated(since = "1.1.7", note = "please use `unsafe_escape` instead")]
/// Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
pub fn unsafe_unescape_owned(base64_url: String) -> String {
    unsafe_unescape(base64_url)
}
