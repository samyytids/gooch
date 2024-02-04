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
    pub direction: Directions,
    pub distance: f32,
}

impl Moving {
    pub fn new(direction: Directions) -> Self {
        Moving {
            direction,
            distance: 0.0,
        }
    }
}