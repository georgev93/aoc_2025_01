
#[derive(Debug, PartialEq)]
pub enum Command {
    Left(u8),
    Right(u8),
}

impl Command {
    fn from_str(s: &str) -> Self {
        let (prefix, number_str) = s.trim_ascii().split_at(1);

        let num = number_str.trim_ascii().parse::<u8>().unwrap_or_else(|_| {
            panic!("Failed to parse {number_str}")
        });
        match prefix {
            "L" => Command::Left(num),
            "R" => Command::Right(num),
            _ => panic!("Not a correct prefix: {prefix}"),
        }
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

    pub fn get_iter(&self) -> core::slice::Iter<'_, Command> {
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
}
