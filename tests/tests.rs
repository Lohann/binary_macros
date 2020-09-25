#[macro_use]
extern crate binary_macros;

#[test]
fn hex_works() {
    assert_eq!(base16!("61"), b"a");
}

#[test]
fn base32hex_works() {
    assert_eq!(base32hex!("C4======"), b"a");
}

#[test]
fn base32_works() {
    assert_eq!(base32!("ME======"), b"a");
}

#[test]
fn base64_works() {
    assert_eq!(base64!("YQ=="), b"a");
}

#[test]
fn base64url_works() {
    assert_eq!(base64url!("_A=="), b"\xfc");
}






#[test]
fn base32hex_nopad_works() {
    assert_eq!(base32hex_nopad!("C4"), b"a");
}

#[test]
fn base32_nopad_works() {
    assert_eq!(base32_nopad!("ME"), b"a");
}

#[test]
fn base64_nopad_works() {
    assert_eq!(base64_nopad!("YQ"), b"a");
}

#[test]
fn base64url_nopad_works() {
    assert_eq!(base64url_nopad!("_A"), b"\xfc");
}




#[test]
fn base64_test() {
    let challenger = std::str::from_utf8(base64!("VGVzdGluZyBCYXNlNjQh")).unwrap();
    let correct = "Testing Base64!";
    assert_eq!(challenger, correct);
}

#[test]
fn include_str() {
    let challenger = base64!("file:tests/test_str.txt");
    let correct = b"Testing include_str!";
    assert_eq!(challenger, correct);
}

#[test]
fn include_envvar() {
    let challenger = base64!("env:BIN_MACROS_TEST");
    let correct = b"Testing dotenv!";
    assert_eq!(challenger, correct);
}

