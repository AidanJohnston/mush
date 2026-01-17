pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyna,
    White,

    FgBlack,
    FgRed,
    FgGreen,
    FgYellow,
    FgBlue,
    FgMagenta,
    FgCyna,
    FgWhite,

    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyna,
    BgWhite,
}
impl Color {
    fn as_term_string(&self) -> &str {
        match self {
            Color::Black => "30m",
            Color::Red => "31m",
            Color::Green => "32m",
            Color::Yellow => "33m",
            Color::Blue => "34m",
            Color::Magenta => "35m",
            Color::Cyna => "36m",
            Color::White => "37m",
            Color::FgBlack => "40m",
            Color::FgRed => "41m",
            Color::FgGreen => "42m",
            Color::FgYellow => "43m",
            Color::FgBlue => "44m",
            Color::FgMagenta => "45m",
            Color::FgCyna => "46m",
            Color::FgWhite => "47m",
            Color::BgBlack => todo!(),
            Color::BgRed => todo!(),
            Color::BgGreen => todo!(),
            Color::BgYellow => todo!(),
            Color::BgBlue => todo!(),
            Color::BgMagenta => todo!(),
            Color::BgCyna => todo!(),
            Color::BgWhite => todo!(),
        }
    }
}

pub enum Effect {
    Bold,
}

const ESCAPE: &str = "\u{001b}[0m";

const BOLD: &str = "\u{001b}[1m";

const BLACK: &str = "\u{001b}[30m";
const RED: &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const YELLOW: &str = "\u{001b}[33m";
const BLUE: &str = "\u{001b}[34m";
const MAGENTA: &str = "\u{001b}[35m";
const CYAN: &str = "\u{001b}[36m";
const WHITE: &str = "\u{001b}[37m";

pub struct ColorString {
    string: String,
}
impl ColorString {
    pub fn new(string: String) -> Self {
        Self { string }
    }

    pub fn color(mut self, color: Color) -> Self {
        self
    }

    pub fn effect(mut self, effect: Effect) -> Self {
        self
    }
}
