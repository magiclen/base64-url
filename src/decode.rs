use crate::alloc::vec::Vec;

/// Decode a Base64-URL string to data.
#[inline]
pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Decode a Base64-URL string to data into a slice and return the valid length.
#[inline]
pub fn decode_to_slice<T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &mut [u8],
) -> Result<usize, base64::DecodeError> {
    base64::decode_config_slice(input, base64::URL_SAFE_NO_PAD, output)
}

/// Decode a Base64-URL string to data and directly store into a Vec instance by concatenating them.
#[inline]
pub fn decode_and_push_to_vec<T: ?Sized + AsRef<[u8]>>(
    input: &T,
    mut output: Vec<u8>,
) -> Result<Vec<u8>, base64::DecodeError> {
    decode_and_push_to_vec_mut(input, &mut output)?;

    Ok(output)
}

/// Decode a Base64-URL string to data and directly store into a mutable Vec reference by concatenating them.
pub fn decode_and_push_to_vec_mut<T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &mut Vec<u8>,
) -> Result<(), base64::DecodeError> {
    let bytes = input.as_ref();

    let current_length = output.len();

    let original_max_length = ((bytes.len() + 3) >> 2) * 3;

    let min_capacity = current_length + original_max_length;

    let capacity = output.capacity();

    if capacity < min_capacity {
        output.reserve(min_capacity - capacity);
    }

    unsafe {
        output.set_len(min_capacity);
    }

    let original_len = decode_to_slice(bytes, &mut output[current_length..min_capacity])?;

    unsafe {
        output.set_len(current_length + original_len);
    }

    Ok(())
}
