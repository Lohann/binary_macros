#[macro_use]
extern crate binary_macros;

#[test]
fn binary_works() {
    assert_eq!(base2!("011000010110000101100001"), b"aaa");
}

#[test]
fn trivial_test_fails() {
    assert_eq!(trivial_test!("abcdefghijklmnopqrstuvwxyz"), "abcdefghijklmnopqrstuvwxyz");
}

#[test]
fn base4_works() {
    assert_eq!(base4!("1201"), b"a");
}

#[test]
fn octal_works() {
    assert_eq!(base8!("302====="), b"a");
}

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
    let challenger = base64!("dotenv:BIN_MACROS_TEST");
    let correct = b"Testing dotenv!";
    assert_eq!(challenger, correct);
}

