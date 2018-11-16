/// Encode data to a Base64-URL string.
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::encode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Encode data to a Base64-URL string into a slice.
pub fn encode_and_store_to_slice<T: ?Sized + AsRef<[u8]>>(input: &T, output: &mut [u8]) -> usize {
    base64::encode_config_slice(input, base64::URL_SAFE_NO_PAD, output)
}

/// Encode data to a Base64-URL string and directly store into a String instance by concatenating them. It is usually for generating a URL.
pub fn encode_and_push_to_string<T: ?Sized + AsRef<[u8]>, S: Into<String>>(input: &T, output: S) -> String {
    let bytes = input.as_ref();

    let mut buffer = output.into().into_bytes();

    let current_len = buffer.len();

    let base64_len = (bytes.len() * 4 + 2) / 3;

    let min_capacity = current_len + base64_len;

    let capacity = buffer.capacity();

    if capacity < min_capacity {
        buffer.reserve(min_capacity - capacity);
    }

    unsafe {
        buffer.set_len(min_capacity);
    }

    let base64_len = encode_and_store_to_slice(bytes, &mut buffer[current_len..min_capacity]);

    unsafe {
        buffer.set_len(current_len + base64_len);
    }

    unsafe {
        String::from_utf8_unchecked(buffer)
    }
}