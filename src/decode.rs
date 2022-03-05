/// Decode a Base64-URL string to data.
#[inline]
pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD)
}

/// Decode a Base64-URL string to data and directly store into a mutable `Vec<u8>` reference by concatenating them and return the slice of the decoded data.
#[inline]
pub fn decode_to_vec<'a, T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &'a mut Vec<u8>,
) -> Result<&'a [u8], base64::DecodeError> {
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
) -> Result<&'a [u8], base64::DecodeError> {
    let length = base64::decode_config_slice(input, base64::URL_SAFE_NO_PAD, output)?;

    Ok(&output[..length])
}

#[deprecated(since = "1.4.0", note = "Please use the `decode_to_vec` function instead")]
/// Decode a Base64-URL string to data and directly store into a Vec instance by concatenating them.
#[inline]
pub fn decode_and_push_to_vec<T: ?Sized + AsRef<[u8]>>(
    input: &T,
    mut output: Vec<u8>,
) -> Result<Vec<u8>, base64::DecodeError> {
    decode_to_vec(input, &mut output)?;

    Ok(output)
}

#[deprecated(since = "1.4.0", note = "Please use the `decode_to_vec` function instead")]
/// Decode a Base64-URL string to data and directly store into a mutable Vec reference by concatenating them.
pub fn decode_and_push_to_vec_mut<T: ?Sized + AsRef<[u8]>>(
    input: &T,
    output: &mut Vec<u8>,
) -> Result<(), base64::DecodeError> {
    decode_to_vec(input, output)?;

    Ok(())
}
