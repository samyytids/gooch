use bevy::prelude::*;
mod animation;
mod setup;
mod player;
mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup::temp_setup)
        .add_systems(
            Update,
            (
                movement::character_movement,
                animation::animate_sprite.after(movement::character_movement)
            )
        )
        .run();
    println!("Hello, world!");
}
