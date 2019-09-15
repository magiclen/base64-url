extern crate base64_url;

#[test]
fn decode() {
    assert_eq!(
        "https://magiclen.org".as_bytes().to_vec(),
        base64_url::decode("aHR0cHM6Ly9tYWdpY2xlbi5vcmc").unwrap()
    );
}

#[test]
fn decode_and_push_to_vec() {
    let url = "https://magiclen.org/".as_bytes().to_vec();

    assert_eq!(
        "https://magiclen.org/articles".as_bytes().to_vec(),
        base64_url::decode_and_push_to_vec("YXJ0aWNsZXM", url).unwrap()
    );
}
