use std::{default::Default, fmt::{Display, write}};

pub enum Extreme<'a> {
    Emoji(&'a str),
}

impl Default for Extreme<'_> {
    fn default() -> Self {
        Self::Emoji("Not Set Yet!")
    }
}

// impl Extreme {
// }
pub enum OneLine<'a> {
    ANSI(&'a str),
    Cool(&'a str),
}

pub enum Colored<'a> {
    Lolcat(&'a str),
    Rgb(&'a str),
    Bg(&'a str),
}

pub enum Fancy<'a> {
    Bloody(&'a str),
    Graphitti(&'a str),
}

pub enum Frame<'a> {
    Asterisk(&'a str),
}
impl Frame {
    fn asterisk(&self) -> &'_ str {
        "anan"
    }
}
impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frame::Asterisk(given) => write(f, Frame::Asterisk(given).asterisk())
        }
    }
}
