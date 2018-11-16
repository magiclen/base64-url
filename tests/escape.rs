extern crate base64_url;

use base64_url::base64;

#[test]
fn unsafe_escape() {
    assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", base64_url::unsafe_escape(base64::encode("https://magiclen.org")));
}