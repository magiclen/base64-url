const CASES: [(&str, &[u8]); 4] =
    [("", b""), ("-w", b"\xfb"), ("-_8", b"\xfb\xff"), ("-___", b"\xfb\xff\xff")];

#[test]
fn decode() {
    assert_eq!(
        b"https://magiclen.org".to_vec(),
        base64_url::decode("aHR0cHM6Ly9tYWdpY2xlbi5vcmc").unwrap()
    );

    for (input, expected) in CASES {
        assert_eq!(expected, base64_url::decode(input).unwrap().as_slice());
    }
}

#[test]
fn decode_to_vec() {
    let mut url = b"https://magiclen.org/".to_vec();

    assert_eq!(b"articles", base64_url::decode_to_vec("YXJ0aWNsZXM", &mut url).unwrap());

    assert_eq!(b"https://magiclen.org/articles", url.as_slice());

    for (input, expected) in CASES {
        let mut output = vec![0, 255];

        assert_eq!(expected, base64_url::decode_to_vec(input, &mut output).unwrap());
        assert_eq!(&[0, 255], &output[..2]);
        assert_eq!(2 + expected.len(), output.len());
    }
}

#[test]
fn decode_to_vec_keeps_output_on_error() {
    let mut output = b"prefix".to_vec();

    assert!(base64_url::decode_to_vec("YWJj-w==", &mut output).is_err());
    assert_eq!(b"prefix", output.as_slice());
}

#[test]
fn decode_to_slice() {
    for (input, expected) in CASES {
        for output_length in [expected.len(), 8] {
            let mut output = [0xAA; 8];

            assert_eq!(
                expected,
                base64_url::decode_to_slice(input, &mut output[..output_length]).unwrap()
            );
            assert_eq!(&[0xAA; 8][expected.len()..], &output[expected.len()..]);
        }
    }
}
