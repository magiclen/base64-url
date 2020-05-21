use crate::alloc::borrow::Cow;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;

/// Unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unsafe_unescape<S: Into<String>>(base64_url: S) -> String {
    let mut base64_url = base64_url.into();

    unsafe_unescape_string(&mut base64_url);

    base64_url
}

/// In-place unescape a Base64-URL string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unsafe_unescape_string(base64_url: &mut String) {
    unsafe_unescape_vec(unsafe { base64_url.as_mut_vec() });
}

/// In-place unescape Base64-URL data to Base64-URL data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
pub fn unsafe_unescape_vec(base64_url: &mut Vec<u8>) {
    for n in base64_url.iter_mut() {
        match *n {
            45 => *n = 43,
            95 => *n = 47,
            _ => (),
        }
    }

    let padding = base64_url.len() % 4;

    if padding > 0 {
        for _ in padding..4 {
            base64_url.push(61);
        }
    }
}

/// Unescape Base64-URL data to Base64-URL data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
pub fn unsafe_unescape_u8_slice<S: ?Sized + AsMut<[u8]>>(base64_url: &mut S) -> Cow<[u8]> {
    let base64_url = base64_url.as_mut();

    for n in base64_url.iter_mut() {
        match *n {
            45 => *n = 43,
            95 => *n = 47,
            _ => (),
        }
    }

    let padding = base64_url.len() % 4;

    if padding > 0 {
        let mut base64_url_vec = base64_url.to_vec();

        for _ in padding..4 {
            base64_url_vec.push(61);
        }

        Cow::from(base64_url_vec)
    } else {
        Cow::from(base64_url as &[u8])
    }
}
