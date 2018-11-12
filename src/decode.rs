/// Decode a Base64-URL string to data.
pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Decode a Base64-URL string to data into a slice.
pub fn decode_and_store_to_slice<T: ?Sized + AsRef<[u8]>>(input: &T, output: &mut [u8]) -> Result<usize, base64::DecodeError> {
    base64::decode_config_slice(input, base64::URL_SAFE_NO_PAD, output)
}

/// Decode a Base64-URL string to data and directly store into a Vec instance by concatenating them.
pub fn decode_and_push_to_vec<T: ?Sized + AsRef<[u8]>>(input: &T, mut output: Vec<u8>) -> Result<Vec<u8>, base64::DecodeError> {
    let bytes = input.as_ref();

    let current_len = output.len();

    let original_max_len = {
        let len = bytes.len();
        (len + 3 / 4) * 3
    };

    let min_capacity = current_len + original_max_len;

    let capacity = output.capacity();

    if capacity < min_capacity {
        output.reserve(min_capacity - capacity);
    }

    unsafe {
        output.set_len(min_capacity);
    }

    let original_len = decode_and_store_to_slice(bytes, &mut output[current_len..min_capacity])?;

    unsafe {
        output.set_len(current_len + original_len);
    }

    Ok(output)
}