use alloc::vec::Vec;

use base64::Engine;

/// Decode a Base64-URL string to data.
#[inline]
pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
}

/// Decode a Base64-URL string to data and directly store into a mutable `Vec<u8>` reference by concatenating them and return the slice of the decoded data.
#[inline]
pub fn decode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut Vec<u8>,
) -> Result<&'a [u8], base64::DecodeSliceError> {
    let bytes = input.as_ref();

    let current_length = output.len();

    let original_max_length = ((bytes.len() + 3) >> 2) * 3;

    output.reserve(original_max_length);

    #[allow(clippy::uninit_vec)]
    unsafe {
        output.set_len(current_length + original_max_length);
    }

    let original_length = decode_to_slice(bytes, &mut output[current_length..])?.len();

    unsafe {
        output.set_len(current_length + original_length);
    }

    Ok(&output[current_length..])
}

/// Decode a Base64-URL string to data into a slice and return the slice with a valid length.
#[inline]
pub fn decode_to_slice<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut [u8],
) -> Result<&'a [u8], base64::DecodeSliceError> {
    let length = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode_slice(input, output)?;

    Ok(&output[..length])
}
