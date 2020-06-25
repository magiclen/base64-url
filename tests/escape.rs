extern crate base64_url;

use base64_url::base64;

#[test]
fn escape() {
    assert_eq!(
        "aHR0cHM6Ly9tYWdpY2xlbi5vcmc",
        base64_url::escape(&base64::encode("https://magiclen.org"))
    );
}

#[test]
fn escape_in_place() {
    let mut base64_string = base64::encode("https://magiclen.org");

    base64_url::escape_in_place(&mut base64_string);

    assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", base64_string);
}
