mod dial;
use crate::dial::Dial;

mod rotations;
use crate::rotations::Rotations;

mod file_parser;
use crate::file_parser::parse_file;

pub fn solve(input_file: &str) -> i32 {
    let mut dial = Dial::new();
    let rotations_iter = Rotations::new(parse_file(input_file)).get_iter();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn example() {
    //     assert_eq!(solve("data/example.txt"), 3);
    // }
}
