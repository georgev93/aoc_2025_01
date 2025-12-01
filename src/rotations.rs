
#[derive(Debug, PartialEq)]
pub enum Command {
    Left(u16),
    Right(u16),
}

impl Command {
    fn from_str(s: &str) -> Self {
        let (prefix, number_str) = s.trim_ascii().split_at(1);

        let num = number_str.trim_ascii().parse::<u16>().unwrap_or_else(|_| {
            panic!("Failed to parse {number_str}")
        });
        match prefix {
            "L" => Command::Left(num),
            "R" => Command::Right(num),
            _ => panic!("Not a correct prefix: {prefix}"),
        }
    }

    pub fn modify(&self, target: &mut u8) {
        let orignal = *target as i16;
        let modifer = match self {
            Command::Left(n) =>  -(*n as i16),
            Command::Right(n) => *n as i16,
        };

        *target = ((((orignal + modifer) % 100) + 100) % 100) as u8;
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
        Command::Left(5).modify(&mut input_val);
        assert_eq!(input_val, 5);

        input_val = 10;
        Command::Left(15).modify(&mut input_val);
        assert_eq!(input_val, 95);

        input_val = 10;
        Command::Right(5).modify(&mut input_val);
        assert_eq!(input_val, 15);

        input_val = 90;
        Command::Right(15).modify(&mut input_val);
        assert_eq!(input_val, 5);

        input_val = 90;
        Command::Right(10).modify(&mut input_val);
        assert_eq!(input_val, 0);
    }
}
