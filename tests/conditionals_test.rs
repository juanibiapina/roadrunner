extern crate hostname;

mod common;
use common::run;

#[test]
fn test_conditionals_empty() {
    assert_eq!(run("{}"), "");
}

#[test]
fn test_conditionals_with_literals_only() {
    assert_eq!(run("{abc()}"), "");
}

#[test]
fn test_conditionals_with_variables() {
    assert_eq!(run("{#{red}}"), "");
}

#[test]
fn test_conditionals_with_trigger() {
    assert_eq!(run("{#{tr(red)}}"), "red");
}

#[test]
fn test_conditionals_nested() {
    assert_eq!(run("{#{red} {#{blue}}}"), "");
}

#[test]
fn test_conditionals_nested_with_trigger() {
    assert_eq!(run("{#{red} {#{tr(blue)}}}"), "red blue");
}
