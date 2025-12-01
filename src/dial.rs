use crate::rotations::Command;
use std::slice::Iter;

pub struct Dial<'a> {
    command_iter: Iter<'a, Command>,
    current_position: u8,
    zero_instances: u32,
    zero_clicks: u32,
}

impl<'a> Dial<'a> {
    pub fn new(command_iter: Iter<'a, Command>) -> Self {
        Self {
            command_iter,
            current_position: 50,
            zero_instances: 0,
            zero_clicks: 0
        }
    }

    pub fn roll_through(&mut self) {
        for command in &mut self.command_iter {
            let old_position = self.current_position;
            println!("Applying command {:?} on {old_position}", command);

            command.modify(&mut self.current_position, &mut self.zero_clicks);

            if self.current_position == 0 {
                self.zero_instances += 1;
            }
            let new_position = self.current_position;
            println!("Changed from {old_position} to {new_position}");
            println!();
        }
    }

    pub fn get_zero_instances(&self) -> u32 {
        self.zero_instances
    }

    pub fn get_zero_clicks(&self) -> u32 {
        self.zero_clicks
    }
}
