extern crate termcolor;
use std::fmt::Display;
use termcolor::Color::*;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct ColorString {
    input: String,
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
}

impl Default for ColorString {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

pub trait ColorTrait
where
    Self: Sized,
{
    fn fg(self, color: Color) -> ColorString;
    fn bg(self, color: Color) -> ColorString;
    fn bold(self) -> ColorString;
    fn italic(self) -> ColorString;
    fn underline(self) -> ColorString;
    #[inline(always)]
    fn green(self) -> ColorString {
        self.fg(Green)
    }
    #[inline(always)]
    fn red(self) -> ColorString {
        self.fg(Red)
    }
    #[inline(always)]
    fn cyan(self) -> ColorString {
        self.fg(Cyan)
    }
    #[inline(always)]
    fn magenta(self) -> ColorString {
        self.fg(Magenta)
    }
    #[inline(always)]
    fn white(self) -> ColorString {
        self.fg(White)
    }
    #[inline(always)]
    fn black(self) -> ColorString {
        self.fg(Black)
    }
    #[inline(always)]
    fn blue(self) -> ColorString {
        self.fg(Blue)
    }
    #[inline(always)]
    fn yellow(self) -> ColorString {
        self.fg(Yellow)
    }
    #[inline(always)]
    fn bg_green(self) -> ColorString {
        self.bg(Green)
    }
    #[inline(always)]
    fn bg_red(self) -> ColorString {
        self.bg(Red)
    }
    #[inline(always)]
    fn bg_cyan(self) -> ColorString {
        self.bg(Cyan)
    }
    #[inline(always)]
    fn bg_magenta(self) -> ColorString {
        self.bg(Magenta)
    }
    #[inline(always)]
    fn bg_white(self) -> ColorString {
        self.bg(White)
    }
    #[inline(always)]
    fn bg_black(self) -> ColorString {
        self.bg(Black)
    }
    #[inline(always)]
    fn bg_blue(self) -> ColorString {
        self.bg(Blue)
    }
    #[inline(always)]
    fn bg_yellow(self) -> ColorString {
        self.bg(Yellow)
    }
}

impl ColorTrait for &str {
    fn fg(self, color: Color) -> ColorString {
        let mut to = ColorString::default();
        to.input = self.to_string();
        to.fg = Some(color);
        to
    }

    fn bg(self, color: Color) -> ColorString {
        let mut to = ColorString::default();
        to.input = self.to_string();
        to.bg = Some(color);
        to
    }

    fn bold(self) -> ColorString {
        let mut to = ColorString::default();
        to.input = self.to_string();
        to.bold = true;
        to
    }

    fn italic(self) -> ColorString {
        let mut to = ColorString::default();
        to.input = self.to_string();
        to.italic = true;
        to
    }

    fn underline(self) -> ColorString {
        let mut to = ColorString::default();
        to.input = self.to_string();
        to.underline = true;
        to
    }
}

impl ColorTrait for ColorString {
    fn fg(mut self, color: Color) -> ColorString {
        self.fg = Some(color);
        self
    }

    fn bg(mut self, color: Color) -> ColorString {
        self.bg = Some(color);
        self
    }

    fn bold(mut self) -> ColorString {
        self.bold = true;
        self
    }

    fn italic(mut self) -> ColorString {
        self.italic = true;
        self
    }

    fn underline(mut self) -> ColorString {
        self.underline = true;
        self
    }
}

impl std::ops::Deref for ColorString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Auto);
        stdout.lock();
        stdout
            .set_color(
                ColorSpec::new()
                    .set_fg(self.fg)
                    .set_bg(self.bg)
                    .set_bold(self.bold)
                    .set_italic(self.italic)
                    .set_underline(self.underline),
            )
            .unwrap_or_default();
        let result = write!(f, "{}", self.input);
        stdout.reset().unwrap_or_default();
        result
    }
}

#[inline(always)]
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Rgb(r, g, b)
}
