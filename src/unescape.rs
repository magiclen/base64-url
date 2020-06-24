use core::mem::swap;
use core::str::from_utf8_unchecked;

use crate::alloc::borrow::Cow;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;

/// Unescape a Base64-URL string to a Base64 string. The conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unescape<S: ?Sized + AsRef<str>>(base64_url: &S) -> Cow<str> {
    let base64_url = base64_url.as_ref();

    match unescape_u8_slice(base64_url) {
        Cow::Owned(base64) => {
            let base64 = unsafe { String::from_utf8_unchecked(base64) };

            Cow::from(base64)
        }
        Cow::Borrowed(base64) => {
            let base64 = unsafe { from_utf8_unchecked(base64) };

            Cow::from(base64)
        }
    }
}

/// Unescape Base64-URL data to Base64 data. The conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
pub fn unescape_u8_slice<S: ?Sized + AsRef<[u8]>>(base64_url: &S) -> Cow<[u8]> {
    let base64_url = base64_url.as_ref();

    let padding = base64_url.len() & 0b11;

    if padding > 0 {
        let mut base64 = Vec::with_capacity(base64_url.len());

        for n in base64_url.iter().copied() {
            match n {
                45 => base64.push(43),
                95 => base64.push(47),
                _ => base64.push(n),
            }
        }

        for _ in padding..4 {
            base64.push(61);
        }

        Cow::from(base64)
    } else {
        let mut need_replace = None;

        for (index, n) in base64_url.iter().enumerate() {
            match n {
                45 | 95 => {
                    need_replace = Some(index);

                    break;
                }
                _ => (),
            }
        }

        match need_replace {
            Some(index) => {
                let mut base64 = Vec::with_capacity(base64_url.len());

                base64.extend_from_slice(&base64_url[..index]);

                for n in base64_url[index..].iter().copied() {
                    match n {
                        45 => base64.push(43),
                        95 => base64.push(47),
                        _ => base64.push(n),
                    }
                }

                Cow::from(base64)
            }
            None => Cow::from(base64_url),
        }
    }
}

/// In-place unescape a Base64-URL string to a Base64 string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unescape_in_place(base64_url: &mut String) -> &str {
    let v = unsafe { base64_url.as_mut_vec() };

    unescape_vec_in_place(v);

    base64_url.as_str()
}

/// In-place unescape Base64-URL data to Base64 data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
#[inline]
pub fn unescape_vec_in_place(base64_url: &mut Vec<u8>) -> &[u8] {
    let base64 = unescape_u8_slice_try_in_place(base64_url.as_mut_slice());

    if let Cow::Owned(mut base64) = base64 {
        swap(base64_url, &mut base64);
    }

    base64_url.as_slice()
}

/// Unescape Base64-URL data to Base64 data and try to do it in-place. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
#[inline]
pub fn unescape_u8_slice_try_in_place<S: ?Sized + AsMut<[u8]>>(base64_url: &mut S) -> Cow<[u8]> {
    let base64_url = base64_url.as_mut();

    for n in base64_url.iter_mut() {
        match *n {
            45 => *n = 43,
            95 => *n = 47,
            _ => (),
        }
    }

    let padding = base64_url.len() & 0b11;

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

#[deprecated(since = "1.3.0", note = "Please use the `unescape_in_place` function instead")]
/// Unescape a Base64-URL string to a Base64 string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unsafe_unescape<S: Into<String>>(base64_url: S) -> String {
    let mut base64_url = base64_url.into();

    unescape_in_place(&mut base64_url);

    base64_url
}

#[deprecated(since = "1.3.0", note = "Please use the `unescape_in_place` function instead")]
/// In-place unescape a Base64-URL string to a Base64 string. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unsafe_unescape_string(base64_url: &mut String) {
    unescape_in_place(base64_url);
}

#[deprecated(since = "1.3.0", note = "Please use the `unescape_vec_in_place` function instead")]
/// In-place unescape Base64-URL data to Base64 data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
pub fn unsafe_unescape_vec(base64_url: &mut Vec<u8>) {
    unescape_vec_in_place(base64_url);
}

#[deprecated(
    since = "1.3.0",
    note = "Please use the `unescape_u8_slice_try_in_place` function instead"
)]
/// Unescape Base64-URL data to Base64 data. It is unsafe because the conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
pub fn unsafe_unescape_u8_slice<S: ?Sized + AsMut<[u8]>>(base64_url: &mut S) -> Cow<[u8]> {
    unescape_u8_slice_try_in_place(base64_url)
}
