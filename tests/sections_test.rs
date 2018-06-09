extern crate hostname;

mod common;
use common::run;

#[test]
fn test_section_no_placeholder() {
    assert_eq!(run("{no placeholder}"), "");
}

#[test]
fn test_section_empty_placeholder() {
    assert_eq!(run("{%nothing%}"), "");
}

#[test]
fn test_section_non_empty_placeholder() {
    assert_eq!(run("{%hostname%}"), hostname::get_hostname().unwrap());
}
