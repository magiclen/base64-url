extern crate base64_url;

use base64_url::base64;

#[test]
fn unescape() {
    assert_eq!(
        b"https://magiclen.org",
        base64::decode(base64_url::unescape("aHR0cHM6Ly9tYWdpY2xlbi5vcmc").as_bytes())
            .unwrap()
            .as_slice()
    );
}

#[test]
fn unescape_in_place() {
    let mut base64_url_string = String::from("aHR0cHM6Ly9tYWdpY2xlbi5vcmc");

    base64_url::unescape_in_place(&mut base64_url_string);

    assert_eq!(b"https://magiclen.org", base64::decode(base64_url_string).unwrap().as_slice());
}
