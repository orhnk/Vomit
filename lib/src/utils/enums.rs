use std::{fmt::{Display}, ops::Deref};

macro_rules! whitespace {
    ($n:expr) => {
        format!("{}{}", " ".repeat($n), "")
    }
}

pub trait Vomitable {}

#[derive(Debug)]
pub struct Food(String);

impl Food {
    pub fn new(given: &mut str) -> Self {
        Food(given.to_string())
    }

    pub fn asterisk(&mut self, margin_hor:usize, margin_ver:usize) {
        let mut base = String::new();
        let basis = self.vomit().len();
        for _ in 0..basis+margin_hor+2 {
            base.push('*');
        }
        for _ in 0..1+margin_ver { // -> now it uses +1 which stands for one line input. I will
            // configure it to be able to procces more than one line. (TODO!)
            base.push_str(&format!("\n*{margin_hor}{self}{margin_hor}*", margin_hor = whitespace!(margin_hor/2), self = self.digest()));
        }
        base.push('\n');
        for _ in 0..basis+margin_hor+2 {
            base.push('*');
        }
        self.0 = base;
    }

    pub fn vomit(&self) -> String {
        (&***self).to_string()
    }
    fn digest<'a>(&'a self) -> &'a str {
        &***self
    }
}

impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.vomit())
    }
}

impl Deref for Food {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

