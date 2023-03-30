use alloc::{string::String, vec::Vec};
use core::str::from_utf8_unchecked;

use base64::Engine;

/// Encode data to a Base64-URL string.
#[inline]
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

/// Encode data to a Base64-URL string and directly store to a mutable `String` reference by concatenating them and return the slice of the Base64-URL string. It is usually for generating a URL.
#[inline]
pub fn encode_to_string<'a, T: ?Sized + AsRef<[u8]>>(input: &T, output: &'a mut String) -> &'a str {
    let base64_url = encode_to_vec(input, unsafe { output.as_mut_vec() });

    unsafe { from_utf8_unchecked(base64_url) }
}

/// Encode data to Base64-URL data and directly store to a mutable `Vec<u8>` reference by concatenating them and return the slice of the Base64-URL data. It is usually for generating a URL.
#[inline]
pub fn encode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(input: &T, output: &'a mut Vec<u8>) -> &'a [u8] {
    let bytes = input.as_ref();

    let current_length = output.len();

    let base64_length = ((bytes.len() << 2) + 2) / 3;

    output.reserve(base64_length);

    #[allow(clippy::uninit_vec)]
    unsafe {
        output.set_len(current_length + base64_length);
    }

    let base64_length = encode_to_slice(bytes, &mut output[current_length..]).unwrap().len();

    unsafe {
        output.set_len(current_length + base64_length);
    }

    &output[current_length..]
}

/// Encode data to a Base64-URL string to a slice and return the slice with a valid length.
#[inline]
pub fn encode_to_slice<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut [u8],
) -> Result<&'a [u8], base64::EncodeSliceError> {
    let length = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode_slice(input, output)?;

    Ok(&output[..length])
}
