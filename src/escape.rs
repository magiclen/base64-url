use crate::alloc::string::String;
use crate::alloc::vec::Vec;

/// Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
#[inline]
pub fn unsafe_escape<S: Into<String>>(base64: S) -> String {
    let mut base64 = base64.into();

    unsafe_escape_string(&mut base64);

    base64
}

/// In-place escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
#[inline]
pub fn unsafe_escape_string(base64: &mut String) {
    unsafe_escape_vec(unsafe { base64.as_mut_vec() });
}

/// In-place escape Base64 data to Base64-URL data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
#[inline]
pub fn unsafe_escape_vec(base64: &mut Vec<u8>) {
    let base64_url_len = unsafe_escape_u8_slice(base64).len();

    unsafe {
        base64.set_len(base64_url_len);
    }
}

/// In-place escape Base64 data to Base64-URL data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
pub fn unsafe_escape_u8_slice<S: ?Sized + AsMut<[u8]>>(base64: &mut S) -> &[u8] {
    let base64 = base64.as_mut();

    let mut len = base64.len();

    for (index, n) in base64.iter_mut().enumerate() {
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

    &base64[..len]
}
