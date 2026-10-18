use std::borrow::Cow;

use base64::Engine;
use base64_url::base64;

#[test]
fn unescape() {
    assert_eq!(
        b"https://magiclen.org",
        base64::engine::general_purpose::STANDARD
            .decode(base64_url::unescape("aHR0cHM6Ly9tYWdpY2xlbi5vcmc").as_bytes())
            .unwrap()
            .as_slice()
    );

    for (input, expected, borrowed) in [
        ("", "", true),
        ("YWJj", "YWJj", true),
        ("YQ", "YQ==", false),
        ("YWI", "YWI=", false),
        ("-w", "+w==", false),
        ("-_8", "+/8=", false),
        ("-___", "+///", false),
        ("AA-w", "AA+w", false),
        ("AAA_", "AAA/", false),
    ] {
        let output = base64_url::unescape(input);

        assert_eq!(expected, output);
        assert_eq!(borrowed, matches!(output, Cow::Borrowed(_)));

        let output = base64_url::unescape_u8_slice(input);

        assert_eq!(expected.as_bytes(), output.as_ref());
        assert_eq!(borrowed, matches!(output, Cow::Borrowed(_)));
    }
}

#[test]
fn unescape_in_place() {
    let mut base64_url_string = String::from("aHR0cHM6Ly9tYWdpY2xlbi5vcmc");

    base64_url::unescape_in_place(&mut base64_url_string);

    assert_eq!(
        b"https://magiclen.org",
        base64::engine::general_purpose::STANDARD.decode(base64_url_string).unwrap().as_slice()
    );

    for (input, expected) in [("-w", "+w=="), ("-_8", "+/8="), ("-___", "+///")] {
        let mut string = String::with_capacity(expected.len());

        string.push_str(input);

        let string_pointer = string.as_ptr();

        assert_eq!(expected, base64_url::unescape_in_place(&mut string));
        assert_eq!(expected, string);
        assert_eq!(string_pointer, string.as_ptr());

        let mut vec = Vec::with_capacity(expected.len());

        vec.extend_from_slice(input.as_bytes());

        let vec_pointer = vec.as_ptr();

        assert_eq!(expected.as_bytes(), base64_url::unescape_vec_in_place(&mut vec));
        assert_eq!(expected.as_bytes(), vec.as_slice());
        assert_eq!(vec_pointer, vec.as_ptr());

        let mut bytes = input.as_bytes().to_vec();
        let output = base64_url::unescape_u8_slice_try_in_place(&mut bytes);

        assert_eq!(expected.as_bytes(), output.as_ref());
        assert_eq!(input.len() % 4 == 0, matches!(output, Cow::Borrowed(_)));
        assert_eq!(&expected.as_bytes()[..input.len()], bytes.as_slice());
    }
}
