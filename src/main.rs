mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;

fn main() {
    println!("Hurrow, Bevy!");

    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.1)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // Game plugins.
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
