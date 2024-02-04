use bevy::prelude::*;
mod animation;
mod setup;
mod player;
mod movement;
mod custom_components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup::temp_setup)
        .add_systems(
            Update,
            (
                movement::check_movement.before(movement::character_movement),
                movement::character_movement,
                animation::animate_sprite.after(movement::character_movement)
            )
        )
        .run();
    println!("Hello, world!");
}
