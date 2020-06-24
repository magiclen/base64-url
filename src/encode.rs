use crate::alloc::string::String;

/// Encode data to a Base64-URL string.
#[inline]
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::encode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Encode data to a Base64-URL string into a slice and return the valid length.
#[inline]
pub fn encode_and_store_to_slice<T: ?Sized + AsRef<[u8]>>(input: &T, output: &mut [u8]) -> usize {
    base64::encode_config_slice(input, base64::URL_SAFE_NO_PAD, output)
}

/// Encode data to a Base64-URL string and directly store into a String instance by concatenating them. It is usually for generating a URL.
#[inline]
pub fn encode_and_push_to_string<T: ?Sized + AsRef<[u8]>, S: Into<String>>(
    input: &T,
    output: S,
) -> String {
    let mut output = output.into();

    encode_and_push_to_string_mut(input, &mut output);

    output
}

/// Encode data to a Base64-URL string and directly store into a mutable String reference by concatenating them. It is usually for generating a URL.
pub fn encode_and_push_to_string_mut<T: ?Sized + AsRef<[u8]>>(input: &T, output: &mut String) {
    let bytes = input.as_ref();

    let buffer = unsafe { output.as_mut_vec() };

    let current_length = buffer.len();

    let base64_length = ((bytes.len() << 2) + 2) / 3;

    let min_capacity = current_length + base64_length;

    let capacity = buffer.capacity();

    if capacity < min_capacity {
        buffer.reserve(min_capacity - capacity);
    }

    unsafe {
        buffer.set_len(min_capacity);
    }

    let base64_len = encode_and_store_to_slice(bytes, &mut buffer[current_length..min_capacity]);

    unsafe {
        buffer.set_len(current_length + base64_len);
    }
}
