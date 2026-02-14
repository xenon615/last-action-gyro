use bevy::prelude::*;
use avian3d::prelude::*;
mod camera;
mod env;
mod field;
mod hero;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        // PhysicsDebugPlugin::default(),
        PhysicsPlugins::default()
    ))
    .add_plugins((
        camera::CameraPlugin,
        env::EnvPlugin,
        field::FieldPlugin,
        hero::HeroPlugin,
    ))
    .run();
}
