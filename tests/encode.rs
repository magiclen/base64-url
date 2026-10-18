const CASES: [(&[u8], &str); 4] =
    [(b"", ""), (b"\xfb", "-w"), (b"\xfb\xff", "-_8"), (b"\xfb\xff\xff", "-___")];

#[test]
fn encode() {
    assert_eq!("aHR0cHM6Ly9tYWdpY2xlbi5vcmc", base64_url::encode("https://magiclen.org"));

    for (input, expected) in CASES {
        assert_eq!(expected, base64_url::encode(input));
    }
}

#[test]
fn encode_to_string() {
    let mut url = "https://magiclen.org/".to_string();

    assert_eq!("YXJ0aWNsZXM", base64_url::encode_to_string("articles", &mut url));

    assert_eq!("https://magiclen.org/YXJ0aWNsZXM", url);

    for (input, expected) in CASES {
        let mut url = "https://magiclen.org/".to_string();

        assert_eq!(expected, base64_url::encode_to_string(input, &mut url));
        assert_eq!(format!("https://magiclen.org/{expected}"), url);
    }
}

#[test]
fn encode_to_vec() {
    for (input, expected) in CASES {
        let mut output = vec![0, 255];

        assert_eq!(expected.as_bytes(), base64_url::encode_to_vec(input, &mut output));
        assert_eq!(&[0, 255], &output[..2]);
        assert_eq!(2 + expected.len(), output.len());
    }
}

#[test]
fn encode_to_slice() {
    for (input, expected) in CASES {
        for output_length in [expected.len(), 8] {
            let mut output = [0xAA; 8];

            assert_eq!(
                expected.as_bytes(),
                base64_url::encode_to_slice(input, &mut output[..output_length]).unwrap()
            );
            assert_eq!(&[0xAA; 8][expected.len()..], &output[expected.len()..]);
        }
    }
}
