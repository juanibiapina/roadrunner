mod common;
use common::run;

#[test]
fn test_variables() {
    assert_eq!(run("#{red}"), "red");
    assert_eq!(run("#{green}"), "green");
    assert_eq!(run("#{blue}"), "blue");
}
