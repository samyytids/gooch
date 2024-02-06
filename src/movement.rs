use crate::*;

use self::collision::check_collision;

pub fn check_movement(
    mut player: Query<(
        &mut player::Player, 
        &mut animation::AnimationBundle, 
        Entity), 
        Without<custom_components::Moving>>,
    player_query: Query<&Transform, With<player::Player>>,
    collider_query: Query<(Entity, &Transform), (With<collision::Collider>, Without<player::Player>)>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (mut player, 
        mut animation_bundle, 
        player_id) 
        in player.iter_mut() {

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

        match check_collision(&player_query, &collider_query, &direction) {
            Some(_) => {moving = false},
            None => (),
        }

        if moving {
            player.animation_state = player::AnimationState::Animating;
            commands.entity(player_id)
            .insert(custom_components::Moving::new(direction));
        }
    }
    
}

pub fn character_movement(
    mut player: Query<(
        & player::Player,
        &mut Transform,
        &animation::AnimationBundle, 
        &mut TextureAtlasSprite, 
        &mut custom_components::Moving, 
        Entity
    )>,
    time:  Res<Time>,
    mut commands: Commands,
) {
    for (player,
        mut transform, 
        animation_bundle, 
        mut texture_atlas, 
        mut moving, 
        player_id) 
        in player.iter_mut() {
        
        let mut movement_amount = player.speed*time.delta_seconds();

        if moving.distance == 16.0*6.0 {
            transform.translation = transform.translation.round();
            commands.entity(player_id).remove::<custom_components::Moving>();
            continue;
        }

        moving.distance += movement_amount;

        // Place these numbers into a resource so I don't have to use magic
        // numbers. 
        if moving.distance > 16.0*6.0 {
            movement_amount = movement_amount - moving.distance % 16.0;
            moving.distance = 16.0*6.0;
        }
        if texture_atlas.index > animation_bundle.indices.last ||
            texture_atlas.index <= animation_bundle.indices.first {
            texture_atlas.index = animation_bundle.indices.first;    
        }

        match moving.direction {
            custom_components::Directions::Up => transform.translation.y += movement_amount,
            custom_components::Directions::Left => transform.translation.x -= movement_amount,
            custom_components::Directions::Down => transform.translation.y -= movement_amount,
            custom_components::Directions::Right => transform.translation.x += movement_amount,
            custom_components::Directions::None => println!("No need to move"),
            _ => println!("movement bug")
        }
    }
    
}