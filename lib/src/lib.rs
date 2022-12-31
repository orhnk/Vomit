mod utils;
use crate::utils::Food;


pub fn run() {
    let mut asterisk_framed_hello = Food::new(&mut "Hello".to_owned()); // -> passing in mutable reference to a String instance.
    asterisk_framed_hello.frame("*", 40, 3); // -> Creating a '*' frame and using some margin values to get the boundary.
    print!("{asterisk_framed_hello}");
}

/*
***********************************************
*                                             *
*                    Hello                    *
*                                             *
***********************************************
*/
