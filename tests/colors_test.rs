mod common;
use common::run;

#[test]
fn test_fg_color_names() {
    assert_eq!(run("#{fg(reset)}"), "\u{1b}[39m");
    assert_eq!(run("#{fg(black)}"), "\u{1b}[38;5;0m");
    assert_eq!(run("#{fg(red)}"), "\u{1b}[38;5;1m");
    assert_eq!(run("#{fg(green)}"), "\u{1b}[38;5;2m");
    assert_eq!(run("#{fg(yellow)}"), "\u{1b}[38;5;3m");
    assert_eq!(run("#{fg(blue)}"), "\u{1b}[38;5;4m");
    assert_eq!(run("#{fg(magenta)}"), "\u{1b}[38;5;5m");
    assert_eq!(run("#{fg(cyan)}"), "\u{1b}[38;5;6m");
    assert_eq!(run("#{fg(white)}"), "\u{1b}[38;5;7m");
}

#[test]
fn test_bg_color_names() {
    assert_eq!(run("#{bg(reset)}"), "\u{1b}[49m");
    assert_eq!(run("#{bg(black)}"), "\u{1b}[48;5;0m");
    assert_eq!(run("#{bg(red)}"), "\u{1b}[48;5;1m");
    assert_eq!(run("#{bg(green)}"), "\u{1b}[48;5;2m");
    assert_eq!(run("#{bg(yellow)}"), "\u{1b}[48;5;3m");
    assert_eq!(run("#{bg(blue)}"), "\u{1b}[48;5;4m");
    assert_eq!(run("#{bg(magenta)}"), "\u{1b}[48;5;5m");
    assert_eq!(run("#{bg(cyan)}"), "\u{1b}[48;5;6m");
    assert_eq!(run("#{bg(white)}"), "\u{1b}[48;5;7m");
}
