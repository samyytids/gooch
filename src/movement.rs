use crate::*;

pub fn character_movement(
    mut player: Query<(&mut Transform, &mut player::Player, &mut animation::AnimationIndices, &mut TextureAtlasSprite,)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut player, mut animation_indices, mut texture_atlas) = player.single_mut();
    let movement_amount = player.speed * time.delta_seconds();

    if input.pressed(KeyCode::W) {
        player.animation_state = player::AnimationState::Animating;
        animation_indices.first = 3;
        animation_indices.last = 5;
        if texture_atlas.index > animation_indices.last || texture_atlas.index <= animation_indices.first {
            texture_atlas.index = animation_indices.first;    
        }
        transform.translation.y += movement_amount;
    }
    if input.pressed(KeyCode::A) {
        animation_indices.first = 6;
        animation_indices.last = 7;
        if texture_atlas.index > animation_indices.last || texture_atlas.index <= animation_indices.first {
            texture_atlas.index = animation_indices.first;    
        }
        player.animation_state = player::AnimationState::Animating;
        transform.translation.x -= movement_amount;
    }
    if input.pressed(KeyCode::S) {
        animation_indices.first = 0;
        animation_indices.last = 2;
        if texture_atlas.index > animation_indices.last || texture_atlas.index <= animation_indices.first {
            texture_atlas.index = animation_indices.first;    
        }
        player.animation_state = player::AnimationState::Animating;
        transform.translation.y -= movement_amount;
    }
    if input.pressed(KeyCode::D) {
        animation_indices.first = 8;
        animation_indices.last = 9;
        if texture_atlas.index > animation_indices.last || texture_atlas.index <= animation_indices.first {
            texture_atlas.index = animation_indices.first;    
        }
        player.animation_state = player::AnimationState::Animating;
        transform.translation.x += movement_amount;
    }
}