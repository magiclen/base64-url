use alloc::vec::Vec;

use base64::Engine;

/// Decode a Base64-URL string to data.
///
/// The input must use the URL-safe alphabet without `=` padding.
#[inline]
pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
}

/// Decode a Base64-URL string, append its decoded bytes to `output`, and return the new bytes.
///
/// The input must use the URL-safe alphabet without `=` padding.
/// On error, the existing bytes and length are kept, but the capacity may grow.
#[inline]
pub fn decode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut Vec<u8>,
) -> Result<&'a [u8], base64::DecodeSliceError> {
    let current_length = output.len();

    // decode_vec zero-fills the new space first; set_len would be faster but breaks Rust's safety rules.
    if let Err(error) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode_vec(input, output) {
        // decode_vec keeps the added space on error, so remove it to restore the original length.
        output.truncate(current_length);

        return Err(error.into());
    }

    Ok(&output[current_length..])
}

/// Decode a Base64-URL string into `output` and return the slice of decoded bytes.
///
/// The input must use the URL-safe alphabet without `=` padding.
/// On success, bytes after the returned slice are left unchanged.
/// Return an error if `output` is too small or the input is invalid.
/// On error, `output` may contain partial decoded data.
#[inline]
pub fn decode_to_slice<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut [u8],
) -> Result<&'a [u8], base64::DecodeSliceError> {
    let length = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode_slice(input, output)?;

    Ok(&output[..length])
}
