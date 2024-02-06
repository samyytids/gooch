use bevy::prelude::*;
use crate::*;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Component)]
pub struct Collider;

pub fn check_collision(
    player_query: &Query<&Transform, With<player::Player>>,
    collider_query: &Query<(Entity, &Transform), (With<Collider>, Without<player::Player>)>,
    direction: &custom_components::Directions,
) -> Option<Collision> {
    // Adjust transform based on direction the player is moving in.
    let player_transform = player_query.single();

    let player_size = player_transform.scale.truncate();
    
    let offset = match direction {
        custom_components::Directions::Up => Vec3::new(0.0, 16.0*6.0, 0.0),
        custom_components::Directions::Down => Vec3::new(0.0, -16.0*6.0, 0.0),
        custom_components::Directions::Left => Vec3::new(-16.0*6.0, 0.0, 0.0),
        custom_components::Directions::Right => Vec3::new(16.0*6.0, 0.0, 0.0),
        custom_components::Directions::None => Vec3::new(0.0, 0.0, 0.0),
    };

    for (collider_entity, transform) in collider_query.iter() {
        let collider_size = transform.scale.truncate();
        let collision = collide(
            player_transform.translation + offset,
            player_size, 
            transform.translation,
            collider_size,
        );

        println!("Collision: {:?}", collision);


        if let Some(collision) = collision {
            return Some(collision)
        }
    }
    None
    
}