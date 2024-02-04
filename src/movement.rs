use crate::*;

pub fn check_movement(
    mut player: Query<(&mut player::Player, &mut animation::AnimationBundle, Entity), Without<custom_components::Moving>>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (mut player, mut animation_bundle, player_id) in player.iter_mut() {

        let mut moving = false;
        let mut direction = custom_components::Directions::None;

        if input.pressed(KeyCode::W) {
            animation_bundle.indices.first = 3;
            animation_bundle.indices.last = 5;
            moving = true;
            direction = custom_components::Directions::Up;
        }
        if input.pressed(KeyCode::A) {
            animation_bundle.indices.first = 6;
            animation_bundle.indices.last = 7;
            moving = true;
            direction = custom_components::Directions::Left;
        }
        if input.pressed(KeyCode::S) {
            animation_bundle.indices.first = 0;
            animation_bundle.indices.last = 2;
            moving = true;
            direction = custom_components::Directions::Down;
        }
        if input.pressed(KeyCode::D) {
            animation_bundle.indices.first = 8;
            animation_bundle.indices.last = 9;
            moving = true;
            direction = custom_components::Directions::Right;
        }
        if moving {
            player.animation_state = player::AnimationState::Animating;
            commands.entity(player_id).insert(custom_components::Moving::new(16*6, direction));
        }
    }
    
}

pub fn character_movement(
    mut player: Query<(&mut Transform, &player::Player, &animation::AnimationBundle, &mut TextureAtlasSprite, &mut custom_components::Moving, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, player, animation_bundle, mut texture_atlas, mut moving, player_id) in player.iter_mut() {
        let movement_amount = player.speed * time.delta_seconds();

        if moving.index == 0 {
            commands.entity(player_id).remove::<custom_components::Moving>();
            continue;
        }

        moving.decrement(1);
        if texture_atlas.index > animation_bundle.indices.last || texture_atlas.index <= animation_bundle.indices.first {
            texture_atlas.index = animation_bundle.indices.first;    
        }

        if moving.direction == custom_components::Directions::Up {
            transform.translation.y += movement_amount;
        }
        if moving.direction == custom_components::Directions::Left {
            transform.translation.x -= movement_amount;
        }
        if moving.direction == custom_components::Directions::Down {
            transform.translation.y -= movement_amount;
        }
        if moving.direction == custom_components::Directions::Right {
            transform.translation.x += movement_amount;
        }
        println!("{}", moving.index);
    }
    
}