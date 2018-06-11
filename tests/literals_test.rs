mod common;
use common::run;

#[test]
fn test_literals_alphanumeric() {
    assert_eq!(run(r#"":) ""#), ":) ");
}

#[test]
fn test_literals_symbols() {
    assert_eq!(run(r#""[@]:()""#), "[@]:()");
}

#[test]
fn test_literals_newline() {
    assert_eq!(run(r#""a\nb ""#), "a\nb ");
}

#[test]
fn test_literals_colors() {
    assert_eq!(run("\"\u{1b}[39mcolor\""), "\u{1b}[39mcolor");
}
