use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Default, Debug)]
pub enum AnimationState {
    Animating,
    #[default]
    Idle,
}

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32, 
    pub animation_state: AnimationState,
}
