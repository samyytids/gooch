use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Default, Debug)]
pub enum AnimationState {
    Animating,
    #[default]
    Idle,
}

#[derive(Component, Debug)]
pub struct NPC {
    pub speed: f32, 
    pub animation_state: AnimationState,
}
