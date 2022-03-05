#[test]
fn encode() {
    assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", base64_url::encode("https://magiclen.org"));
}

#[test]
fn encode_to_string() {
    let mut url = "https://magiclen.org/".to_string();

    assert_eq!("YXJ0aWNsZXM", base64_url::encode_to_string("articles", &mut url));

    assert_eq!("https://magiclen.org/YXJ0aWNsZXM", url);
}
