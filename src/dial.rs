use crate::rotations::Command;
use std::slice::Iter;

pub struct Dial<'a> {
    command_iter: Iter<'a, Command>,
    current_position: u8,
    zero_instances: u32,
}

impl<'a> Dial<'a> {
    pub fn new(command_iter: Iter<'a, Command>) -> Self {
        Self {
            command_iter,
            current_position: 50,
            zero_instances: 0,
        }
    }

    pub fn roll_through(&mut self) {
        for command in &mut self.command_iter {
            let old_position = self.current_position;

            command.modify(&mut self.current_position);

            if self.current_position == 0 {
                self.zero_instances += 1;
            }
            let new_position = self.current_position;
            println!("Applied command {:?} to change {old_position} to {new_position}", command);
        }
    }

    pub fn get_zero_instances(&self) -> u32 {
        self.zero_instances
    }
}
