mod dial;
use crate::dial::Dial;

mod rotations;
use crate::rotations::Rotations;

mod file_parser;
use crate::file_parser::parse_file;

pub fn solve(input_file: &str) -> (u32, u32) {
    let rotations = Rotations::new(parse_file(input_file));
    let mut dial = Dial::new(rotations.get_iter());
    dial.roll_through();
    (dial.get_zero_instances(), dial.get_zero_clicks())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (part_1, part_2) = solve("data/example.txt");
        assert_eq!(part_1, 3);
        assert_eq!(part_2, 6);
    }
}
