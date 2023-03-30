use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{mem::swap, str::from_utf8_unchecked};

/// Unescape a Base64-URL string to a Base64 string. The conversion is not concerning with Base64 decoding. You need to make sure the input string is a correct Base64-URL string by yourself.
#[inline]
pub fn unescape<S: ?Sized + AsRef<str>>(base64_url: &S) -> Cow<str> {
    let base64_url = base64_url.as_ref();

    match unescape_u8_slice(base64_url) {
        Cow::Owned(base64) => {
            let base64 = unsafe { String::from_utf8_unchecked(base64) };

            Cow::from(base64)
        },
        Cow::Borrowed(base64) => {
            let base64 = unsafe { from_utf8_unchecked(base64) };

            Cow::from(base64)
        },
    }
}

/// Unescape Base64-URL data to Base64 data. The conversion is not concerning with Base64 decoding. You need to make sure the input Base64-URL data is correct by yourself.
pub fn unescape_u8_slice<S: ?Sized + AsRef<[u8]>>(base64_url: &S) -> Cow<[u8]> {
    let base64_url = base64_url.as_ref();
    let length = base64_url.len();

    let padding = length & 0b11;

    if padding > 0 {
        let new_size = base64_url.len() + (4 - padding);

        let mut base64 = Vec::with_capacity(new_size);

        let mut p = 0;
        let mut start = 0;

        loop {
            if p == length {
                break;
            }

            let e = base64_url[p];

            match e {
                45 => {
                    base64.extend_from_slice(&base64_url[start..p]);
                    start = p + 1;

                    base64.push(43);
                },
                95 => {
                    base64.extend_from_slice(&base64_url[start..p]);
                    start = p + 1;

                    base64.push(47);
                },
                _ => (),
            }

            p += 1;
        }

        base64.extend_from_slice(&base64_url[start..p]);

        base64.resize(new_size, 61);

        Cow::from(base64)
    } else {
        let mut p = 0;

        let first = loop {
            if p == length {
                return Cow::from(base64_url);
            }

            let e = base64_url[p];

            match e {
                45 => {
                    break 43;
                },
                95 => {
                    break 47;
                },
                _ => (),
            }

            p += 1;
        };

        let mut base64 = Vec::with_capacity(base64_url.len());

        base64.extend_from_slice(&base64_url[..p]);
        base64.push(first);

        let mut start = p;
        p += 1;

        loop {
            if p == length {
                break;
            }

            let e = base64_url[p];

            match e {
                45 => {
                    base64.extend_from_slice(&base64_url[start..p]);
                    start = p + 1;

                    base64.push(43);
                },
                95 => {
                    base64.extend_from_slice(&base64_url[start..p]);
                    start = p + 1;

                    base64.push(47);
                },
                _ => (),
            }

            p += 1;
        }

        base64.extend_from_slice(&base64_url[start..p]);

        Cow::from(base64)
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

        let new_size = base64_url.len() + (4 - padding);

        base64_url_vec.resize(new_size, 61);

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
