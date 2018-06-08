mod common;
use common::run;

#[test]
fn test_literals_alphanumeric() {
    assert_eq!(run("some text"), "some text");
}

#[test]
fn test_literals_symbols() {
    assert_eq!(run("[@]:()"), "[@]:()");
}

#[test]
fn test_literals_newline() {
    assert_eq!(run("a\nb "), "a\nb ");
}
