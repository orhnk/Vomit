enum Extreme {
    Emoji,
}

enum OneLine {
    ANSI,
    Cool,
}

enum Colored {
    Lolcat,
    Rgb,
    Bg,
}

enum Fancy {
    Bloody,
    Graphitti,
}

enum Frame {
    Asteriks,
}

pub trait Vomit {
    fn bloody(food: &str) -> &str;
    fn graphitti(food: &str) -> &str;
    fn lolcat(food: &str) -> &str;
    fn x(food: &str) -> &str;
}

pub fn run() {
    let shit = add(1, 2);
    print!("shit is {shit}");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
