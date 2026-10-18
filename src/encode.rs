use alloc::{string::String, vec::Vec};
use core::str::from_utf8_unchecked;

use base64::Engine;

/// Encode data to a Base64-URL string without `=` padding.
#[inline]
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

/// Append data as Base64-URL text without `=` padding to `output` and return the new text.
///
/// This function uses [`encode_to_vec`] and has the same unsafe buffer handling described there.
#[inline]
pub fn encode_to_string<'a, T: ?Sized + AsRef<[u8]>>(input: &T, output: &'a mut String) -> &'a str {
    let base64_url = encode_to_vec(input, unsafe { output.as_mut_vec() });

    unsafe { from_utf8_unchecked(base64_url) }
}

/// Append data as Base64-URL bytes without `=` padding to `output` and return the new bytes.
///
/// # Implementation notes
///
/// This function uses `set_len` before encoding to avoid an extra memory write.
/// With `base64` 0.23.1, the encoder writes new output bytes without reading their old values.
/// Earlier project benchmarks found that using safe APIs was slower, so this path is kept for speed.
/// This still does not meet the initialization rule of `Vec::set_len`; benchmark results do not prove memory safety.
#[inline]
pub fn encode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(input: &T, output: &'a mut Vec<u8>) -> &'a [u8] {
    let bytes = input.as_ref();

    let current_length = output.len();

    let base64_length =
        base64::encoded_len(bytes.len(), false).expect("Base64-URL length is too large");

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
