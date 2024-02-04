use bevy::prelude::*;
use crate::*;


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
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut player::Player,
    )>,
) {
    for (indices, mut timer, mut sprite, mut player) in &mut query {
        if player.animation_state == player::AnimationState::Idle {
            continue
        }
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                player.animation_state = player::AnimationState::Idle;
                indices.first
            } else {
                sprite.index + 1
            }
        };
        println!("index: {}, First: {}, Last: {}", sprite.index, indices.first, indices.last);
    }
}