extern crate hostname;

mod common;
use common::run;

#[test]
fn test_sections_only_one() {
    assert_eq!(run("single part"), "single part");
}

#[test]
fn test_sections_several() {
    assert_eq!(run("part1;part2;part3"), "part1part2part3");
}
