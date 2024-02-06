use bevy::prelude::*;
use crate::*;

pub fn temp_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("MC.png");
    let texture_atlas = 
        TextureAtlas::from_grid(
            texture_handle, Vec2::new(16.0,16.0), 10, 3, None, None
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = animation::AnimationIndices {
        first: 0,
        last: 2
    };

    let texture = asset_server.load("tiledbg.png");

    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture,
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation::AnimationBundle {
            indices: animation_indices,
            timer: animation::AnimationTimer(
                Timer::from_seconds(0.2, TimerMode::Repeating)
            ),
        },
        player::Player { 
            speed: 200.0,
            animation_state: default(),
        },
        collision::Collider,
    ));

    let animation_indices = animation::AnimationIndices {
        first: 0,
        last: 2
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform{
                translation: Vec3::new(16.0*6.0, 16.0*6.0, 7.0),
                scale: Vec3::splat(6.0),
                ..default()
            },
            ..default()
        },
        animation::AnimationBundle {
            indices: animation_indices,
            timer: animation::AnimationTimer(
                Timer::from_seconds(0.2, TimerMode::Repeating)
            ),
        },
        npcs::NPC { 
            speed: 200.0,
            animation_state: default(),
        },
        collision::Collider,
    ));
}