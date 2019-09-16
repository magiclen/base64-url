extern crate base64_url;

use base64_url::base64;

#[test]
fn unsafe_unescape() {
    assert_eq!(
        b"https://magiclen.org".to_vec(),
        base64::decode(&base64_url::unsafe_unescape("aHR0cHM6Ly9tYWdpY2xlbi5vcmc")).unwrap()
    );
}
