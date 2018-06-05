mod common;
use common::run;

#[test]
fn test_fg_color_names() {
    assert_eq!(run("#{fg:%reset%}text"), "\u{1b}[39mtext");
    assert_eq!(run("#{fg:%black%}text"), "\u{1b}[38;5;0mtext");
    assert_eq!(run("#{fg:%red%}text"), "\u{1b}[38;5;1mtext");
    assert_eq!(run("#{fg:%green%}text"), "\u{1b}[38;5;2mtext");
    assert_eq!(run("#{fg:%yellow%}text"), "\u{1b}[38;5;3mtext");
    assert_eq!(run("#{fg:%blue%}text"), "\u{1b}[38;5;4mtext");
    assert_eq!(run("#{fg:%magenta%}text"), "\u{1b}[38;5;5mtext");
    assert_eq!(run("#{fg:%cyan%}text"), "\u{1b}[38;5;6mtext");
    assert_eq!(run("#{fg:%white%}text"), "\u{1b}[38;5;7mtext");
}

#[test]
fn test_bg_color_names() {
    assert_eq!(run("#{bg:%reset%}text"), "\u{1b}[49mtext");
    assert_eq!(run("#{bg:%black%}text"), "\u{1b}[48;5;0mtext");
    assert_eq!(run("#{bg:%red%}text"), "\u{1b}[48;5;1mtext");
    assert_eq!(run("#{bg:%green%}text"), "\u{1b}[48;5;2mtext");
    assert_eq!(run("#{bg:%yellow%}text"), "\u{1b}[48;5;3mtext");
    assert_eq!(run("#{bg:%blue%}text"), "\u{1b}[48;5;4mtext");
    assert_eq!(run("#{bg:%magenta%}text"), "\u{1b}[48;5;5mtext");
    assert_eq!(run("#{bg:%cyan%}text"), "\u{1b}[48;5;6mtext");
    assert_eq!(run("#{bg:%white%}text"), "\u{1b}[48;5;7mtext");
}
