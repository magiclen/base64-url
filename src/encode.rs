use alloc::{string::String, vec::Vec};
use core::str::from_utf8_unchecked;

use base64::Engine;

/// Encode data to a Base64-URL string without `=` padding.
#[inline]
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

/// Append data as Base64-URL text without `=` padding to `output` and return the new text.
#[inline]
pub fn encode_to_string<'a, T: ?Sized + AsRef<[u8]>>(input: &T, output: &'a mut String) -> &'a str {
    // SAFETY: Only zero or Base64 ASCII bytes are added, so the output and returned slice stay valid UTF-8.
    let base64_url = encode_to_vec(input, unsafe { output.as_mut_vec() });

    unsafe { from_utf8_unchecked(base64_url) }
}

/// Append data as Base64-URL bytes without `=` padding to `output` and return the new bytes.
#[inline]
pub fn encode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(input: &T, output: &'a mut Vec<u8>) -> &'a [u8] {
    let bytes = input.as_ref();

    let current_length = output.len();

    let base64_length =
        base64::encoded_len(bytes.len(), false).expect("Base64-URL length is too large");

    let new_length =
        current_length.checked_add(base64_length).expect("Base64-URL length is too large");

    // Zero-fill the new space first because encode_to_slice needs initialized memory; set_len would be faster but breaks Rust's safety rules.
    output.resize(new_length, 0);

    encode_to_slice(bytes, &mut output[current_length..]).unwrap()
}

/// Encode data to Base64-URL bytes without `=` padding and return the written part of `output`.
///
/// Bytes after the returned slice are left unchanged.
/// Return an error if `output` is too small.
#[inline]
pub fn encode_to_slice<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut [u8],
) -> Result<&'a [u8], base64::EncodeSliceError> {
    let length = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode_slice(input, output)?;

    Ok(&output[..length])
}
