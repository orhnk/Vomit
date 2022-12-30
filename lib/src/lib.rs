mod utils;
use utils::enums::Frame;

use crate::utils::{enums::Extreme};

pub fn run() {
    let asterisk = Frame::Asterisk("Some");
    print!("{}", asterisk);
}
