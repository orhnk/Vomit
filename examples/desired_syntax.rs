use vomit::Food;


fn main() {
    let asterisk_framed_hello = Food::new("Hello");
    asterisk_framed_hello.frame("*", 40, 3);
    print!("{asterisk_framed_hello}");
}

/*
********************************
*                              *
*            hello             *
*                              *
********************************
*/
