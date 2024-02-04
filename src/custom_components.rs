use bevy::prelude::*;

#[derive(PartialEq)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Component)]
pub struct Moving {
    pub index: u8,
    pub direction: Directions,
}



impl Moving {
    pub fn decrement(&mut self, amount: u8) {
        self.index -= amount;
    }

    pub fn new(index: u8, direction: Directions) -> Self {
        Moving {
            index,
            direction,
        }
    }
}