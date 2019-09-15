/// Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
pub fn unsafe_escape<S: Into<String>>(base64: S) -> String {
    let mut result = base64.into().into_bytes();

    let mut len = result.len();

    for (index, n) in result.iter_mut().enumerate() {
        match *n {
            43 => *n = 45,
            47 => *n = 95,
            61 => {
                len = index;
                break;
            }
            _ => (),
        }
    }

    unsafe {
        result.set_len(len);
    }

    unsafe {
        String::from_utf8_unchecked(result)
    }
}

#[deprecated(since = "1.1.7", note = "please use `unsafe_escape` instead")]
/// Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
pub fn unsafe_escape_owned(base64: String) -> String {
    unsafe_escape(base64)
}
