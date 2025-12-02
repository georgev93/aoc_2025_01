#[derive(Debug, PartialEq)]
pub enum Command {
    Left(u16),
    Right(u16),
}

impl Command {
    fn from_str(s: &str) -> Self {
        let (prefix, number_str) = s.trim_ascii().split_at(1);

        let num = number_str
            .trim_ascii()
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("Failed to parse {number_str}"));
        match prefix {
            "L" => Command::Left(num),
            "R" => Command::Right(num),
            _ => panic!("Not a correct prefix: {prefix}"),
        }
    }

    pub fn modify(&self, target: &mut u8, zero_clicks: &mut u32) {
        let original = *target as i16;
        let modifer = match self {
            Command::Left(n) => -(*n as i16),
            Command::Right(n) => *n as i16,
        };

        let mut sum = original + modifer;
        let mut zero_click_modification: i16 = 0;

        if original == 0 && sum < 0 {
            zero_click_modification = -1;
        }

        while sum < 0 {
            sum += 100;
            // println!("Click! Was negative");
            zero_click_modification += 1;
        }

        if sum == 0 {
            // println!("Click! Landed on 0");
            zero_click_modification += 1;
        }

        while sum > 99 {
            // println!("Click! Was over 100");
            sum -= 100;
            zero_click_modification += 1;
        }

        *target = sum as u8;
        *zero_clicks += zero_click_modification as u32;

    }
}

pub struct Rotations {
    commands: Vec<Command>,
}

impl Rotations {
    pub fn new(command_strings: Vec<String>) -> Self {
        Self {
            commands: command_strings
                .iter()
                .map(|s| Command::from_str(s))
                .collect(),
        }
    }

    pub fn get_iter(&self) -> std::slice::Iter<'_, Command> {
        self.commands.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_command_parsing() {
        assert_eq!(Command::Left(1), Command::from_str("L1"));
        assert_eq!(Command::Right(1), Command::from_str("R1"));
        assert_eq!(Command::Right(99), Command::from_str("R99"));
        assert_eq!(Command::Right(99), Command::from_str("R99\n"));
        assert_eq!(Command::Right(99), Command::from_str("\n \n \n  R99\n"));
    }

    #[test]
    fn check_modify() {
        let mut input_val: u8 = 10;
        let mut zero_clicks = 0;
        Command::Left(5).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 5);
        assert_eq!(zero_clicks, 0);

        input_val = 10;
        zero_clicks = 0;
        Command::Left(15).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 95);
        assert_eq!(zero_clicks, 1);

        input_val = 10;
        zero_clicks = 0;
        Command::Left(10).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 0);
        assert_eq!(zero_clicks, 1);

        input_val = 10;
        zero_clicks = 0;
        Command::Right(5).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 15);
        assert_eq!(zero_clicks, 0);

        input_val = 90;
        zero_clicks = 0;
        Command::Right(15).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 5);
        assert_eq!(zero_clicks, 1);

        input_val = 90;
        zero_clicks = 0;
        Command::Right(10).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 0);
        assert_eq!(zero_clicks, 1);

        input_val = 0;
        zero_clicks = 0;
        Command::Left(5).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 95);
        assert_eq!(zero_clicks, 0);

        input_val = 0;
        zero_clicks = 0;
        Command::Left(100).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 0);
        assert_eq!(zero_clicks, 1);

        input_val = 0;
        zero_clicks = 0;
        Command::Left(200).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 0);
        assert_eq!(zero_clicks, 2);

        input_val = 0;
        zero_clicks = 0;
        Command::Right(100).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 0);
        assert_eq!(zero_clicks, 1);

        input_val = 0;
        zero_clicks = 0;
        Command::Right(200).modify(&mut input_val, &mut zero_clicks);
        assert_eq!(input_val, 0);
        assert_eq!(zero_clicks, 2);
    }
}
