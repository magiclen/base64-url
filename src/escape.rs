use core::str::from_utf8_unchecked;

use crate::alloc::borrow::Cow;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;

/// Escape a Base64 string to a Base64-URL string. The conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
#[inline]
pub fn escape<S: ?Sized + AsRef<str>>(base64: &S) -> Cow<str> {
    let base64 = base64.as_ref();

    match escape_u8_slice(base64) {
        Cow::Owned(base64_url) => {
            let base64_url = unsafe { String::from_utf8_unchecked(base64_url) };

            Cow::from(base64_url)
        }
        Cow::Borrowed(base64_url) => {
            let base64_url = unsafe { from_utf8_unchecked(base64_url) };

            Cow::from(base64_url)
        }
    }
}

/// Escape Base64 data to Base64-URL data. The conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
#[inline]
pub fn escape_u8_slice<S: ?Sized + AsRef<[u8]>>(base64: &S) -> Cow<[u8]> {
    let base64 = base64.as_ref();

    let mut need_replace = None;

    for (index, n) in base64.iter().enumerate() {
        match n {
            43 | 47 => {
                need_replace = Some(index);

                break;
            }
            61 => {
                return Cow::from(&base64[..index]);
            }
            _ => (),
        }
    }

    match need_replace {
        Some(index) => {
            let mut base64_url = Vec::with_capacity(base64.len());

            base64_url.extend_from_slice(&base64[..index]);

            for n in base64[index..].iter().copied() {
                match n {
                    43 => base64_url.push(45),
                    47 => base64_url.push(95),
                    61 => break,
                    _ => base64_url.push(n),
                }
            }

            Cow::from(base64_url)
        }
        None => Cow::from(base64),
    }
}

/// In-place escape a Base64 string to a Base64-URL string. The conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
#[inline]
pub fn escape_in_place(base64: &mut String) -> &str {
    let v = unsafe { base64.as_mut_vec() };

    escape_vec_in_place(v);

    base64.as_str()
}

/// In-place escape Base64 data to Base64-URL data. The conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
#[inline]
pub fn escape_vec_in_place(base64: &mut Vec<u8>) -> &[u8] {
    let length = escape_u8_slice_in_place(base64.as_mut_slice()).len();

    unsafe {
        base64.set_len(length);
    }

    base64.as_slice()
}

/// In-place escape Base64 data to Base64-URL data. The conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
#[inline]
pub fn escape_u8_slice_in_place<S: ?Sized + AsMut<[u8]>>(base64: &mut S) -> &[u8] {
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

#[deprecated(since = "1.3.0", note = "Please use the `escape_in_place` function instead")]
/// Escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
#[inline]
pub fn unsafe_escape<S: Into<String>>(base64: S) -> String {
    let mut base64 = base64.into();

    escape_in_place(&mut base64);

    base64
}

#[deprecated(since = "1.3.0", note = "Please use the `escape_in_place` function instead")]
/// In-place escape a Base64 string to a Base64-URL string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64 string by yourself.
#[inline]
pub fn unsafe_escape_string(base64: &mut String) {
    escape_in_place(base64);
}

#[deprecated(since = "1.3.0", note = "Please use the `escape_vec_in_place` function instead")]
/// In-place escape Base64 data to Base64-URL data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
#[inline]
pub fn unsafe_escape_vec(base64: &mut Vec<u8>) {
    escape_vec_in_place(base64);
}

#[deprecated(since = "1.3.0", note = "Please use the `escape_u8_slice_in_place` function instead")]
/// In-place escape Base64 data to Base64-URL data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64 data is correct by yourself.
#[inline]
pub fn unsafe_escape_u8_slice<S: ?Sized + AsMut<[u8]>>(base64: &mut S) -> &[u8] {
    escape_u8_slice_in_place(base64)
}
