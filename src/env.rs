use bevy::prelude::*;
pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
        ;
    }
}

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        DirectionalLight{
            illuminance: 14e4,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
