extern crate base64_url;

#[test]
fn encode() {
    assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", base64_url::encode("https://magiclen.org"));
}

#[test]
fn encode_and_push_to_string() {
    let url = "https://magiclen.org/".to_string();

    assert_eq!("https://magiclen.org/YXJ0aWNsZXM", base64_url::encode_and_push_to_string("articles", url));
}