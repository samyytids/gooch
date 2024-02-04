use bevy::prelude::*;
use crate::*;

// Create an animation bundle so that I can keep the information in one place
// Making it easier to deal with animation incrementation. General idea at this
// point is to increment the index uniformly across the length of the animation
// for example a 16 frame long animation with 4 frames of animation will 
// increment once ever 4 frames. 
// I will however need to figure out how to handle changing what animation is
// in the bundle. Probably have a function that takes the current state and 
// uses some kind of hash map to tie animation to what is currently happening
// in the game. 

#[derive(Component)]
pub struct AnimationBundle {
    pub indices: AnimationIndices, // Have these be mutable based on the state
    pub timer: AnimationTimer, // Have these be mutable based on the state
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize, 
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationBundle,
        &mut TextureAtlasSprite,
        &mut player::Player,
    )>,
) {
    for (mut animation_bundle, mut sprite, mut player) in &mut query {

        if player.animation_state == player::AnimationState::Idle {
            continue
        }
        
        animation_bundle.timer.tick(time.delta());
        if animation_bundle.timer.just_finished() {
            sprite.index = if sprite.index == animation_bundle.indices.last {
                player.animation_state = player::AnimationState::Idle;
                animation_bundle.indices.first
            } else {
                sprite.index + 1
            }
        };
    }
}