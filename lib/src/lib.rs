mod utils;
use utils::enums::Food;

pub fn run() {
    let mut asterisk = Food::new(&mut "Works Fine but My exam week is coming :(".to_owned());
    asterisk.asterisk(110, 10);
    print!("{}", asterisk.vomit());
}
