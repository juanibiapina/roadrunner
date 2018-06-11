extern crate termion;
extern crate rlua;

use self::rlua::UserData;

#[derive(Clone)]
pub enum ColorName {
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    White,
    Yellow,
}

#[derive(Clone)]
pub enum Color {
    Name(ColorName),
    Reset,
}

impl UserData for Color {
}

pub fn fg(color: Color) -> String {
    match color {
        Color::Reset => format!("{}", termion::color::Fg(termion::color::Reset)),
        Color::Name(name) => {
            match name {
                ColorName::Black => format!("{}", termion::color::Fg(termion::color::Black)),
                ColorName::Blue => format!("{}", termion::color::Fg(termion::color::Blue)),
                ColorName::Cyan => format!("{}", termion::color::Fg(termion::color::Cyan)),
                ColorName::Green => format!("{}", termion::color::Fg(termion::color::Green)),
                ColorName::LightBlack => format!("{}", termion::color::Fg(termion::color::LightBlack)),
                ColorName::LightBlue => format!("{}", termion::color::Fg(termion::color::LightBlue)),
                ColorName::LightCyan => format!("{}", termion::color::Fg(termion::color::LightCyan)),
                ColorName::LightGreen => format!("{}", termion::color::Fg(termion::color::LightGreen)),
                ColorName::LightMagenta => format!("{}", termion::color::Fg(termion::color::LightMagenta)),
                ColorName::LightRed => format!("{}", termion::color::Fg(termion::color::LightRed)),
                ColorName::LightWhite => format!("{}", termion::color::Fg(termion::color::LightWhite)),
                ColorName::LightYellow => format!("{}", termion::color::Fg(termion::color::LightYellow)),
                ColorName::Magenta => format!("{}", termion::color::Fg(termion::color::Magenta)),
                ColorName::Red => format!("{}", termion::color::Fg(termion::color::Red)),
                ColorName::White => format!("{}", termion::color::Fg(termion::color::White)),
                ColorName::Yellow => format!("{}", termion::color::Fg(termion::color::Yellow)),
            }
        }
    }
}

pub fn bg(color: Color) -> String {
    match color {
        Color::Reset => format!("{}", termion::color::Bg(termion::color::Reset)),
        Color::Name(name) => {
            match name {
                ColorName::Black => format!("{}", termion::color::Bg(termion::color::Black)),
                ColorName::Blue => format!("{}", termion::color::Bg(termion::color::Blue)),
                ColorName::Cyan => format!("{}", termion::color::Bg(termion::color::Cyan)),
                ColorName::Green => format!("{}", termion::color::Bg(termion::color::Green)),
                ColorName::LightBlack => format!("{}", termion::color::Bg(termion::color::LightBlack)),
                ColorName::LightBlue => format!("{}", termion::color::Bg(termion::color::LightBlue)),
                ColorName::LightCyan => format!("{}", termion::color::Bg(termion::color::LightCyan)),
                ColorName::LightGreen => format!("{}", termion::color::Bg(termion::color::LightGreen)),
                ColorName::LightMagenta => format!("{}", termion::color::Bg(termion::color::LightMagenta)),
                ColorName::LightRed => format!("{}", termion::color::Bg(termion::color::LightRed)),
                ColorName::LightWhite => format!("{}", termion::color::Bg(termion::color::LightWhite)),
                ColorName::LightYellow => format!("{}", termion::color::Bg(termion::color::LightYellow)),
                ColorName::Magenta => format!("{}", termion::color::Bg(termion::color::Magenta)),
                ColorName::Red => format!("{}", termion::color::Bg(termion::color::Red)),
                ColorName::White => format!("{}", termion::color::Bg(termion::color::White)),
                ColorName::Yellow => format!("{}", termion::color::Bg(termion::color::Yellow)),
            }
        }
    }
}
