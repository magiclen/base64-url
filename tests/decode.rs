extern crate base64_url;

#[test]
fn decode() {
    assert_eq!(
        b"https://magiclen.org".to_vec(),
        base64_url::decode("aHR0cHM6Ly9tYWdpY2xlbi5vcmc").unwrap()
    );
}

#[test]
fn decode_and_push_to_vec() {
    let mut url = b"https://magiclen.org/".to_vec();

    assert_eq!(b"articles", base64_url::decode_to_vec("YXJ0aWNsZXM", &mut url).unwrap());

    assert_eq!(b"https://magiclen.org/articles", url.as_slice());
}
