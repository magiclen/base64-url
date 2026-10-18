use alloc::vec::Vec;
use core::slice::from_raw_parts_mut;

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
///
/// # Implementation notes
///
/// This function uses `set_len` after a successful decode to include only the bytes written.
/// With `base64` 0.23.1, the decoder writes new output bytes without reading their old values.
/// Earlier project benchmarks found that using safe APIs was slower, so this path is kept for speed.
/// This still does not meet the initialization rule of `core::slice::from_raw_parts_mut`; benchmark results do not prove memory safety.
#[inline]
pub fn decode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut Vec<u8>,
) -> Result<&'a [u8], base64::DecodeSliceError> {
    let bytes = input.as_ref();

    let current_length = output.len();

    let original_max_length = ((bytes.len() + 3) >> 2) * 3;

    output.reserve(original_max_length);

    let original_length = {
        let spare = output.spare_capacity_mut();

        let buf: &mut [u8] =
            unsafe { from_raw_parts_mut(spare.as_mut_ptr().cast::<u8>(), original_max_length) };

        decode_to_slice(bytes, buf)?.len()
    };

    #[allow(clippy::uninit_vec)]
    unsafe {
        output.set_len(current_length + original_length);
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
