use bevy::{
    // camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    core_pipeline::Skybox,
    prelude::*
};

use avian3d::prelude::{LinearVelocity, PhysicsSystems};

use crate::{
    hero::Body
};


pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate,
            follow
            .after(PhysicsSystems::Writeback)
            .before(TransformSystems::Propagate)
        )
        ;
    }
}

// ---

#[derive(Component)]
pub struct Cam;

#[derive(Resource)]
pub struct CamFollowParams {
    pub tranlation_bias: Vec3,
    pub look_bias: Vec3,
    pub translation_speed: f32,
    pub rotation_speed: f32
}

// ---

fn setup (
    mut cmd: Commands,
    assets: ResMut<AssetServer>,
) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(30., 20., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        Cam,
        Camera::default(),
        // FreeCamera::default(),
        Skybox {
            image: assets.load("skyboxes/space-bw.ktx2"),
            brightness: 500.,
            ..default()
        },
    ));

    cmd.insert_resource(
        CamFollowParams{
            tranlation_bias: Vec3::new(0., 13., 50.),
            look_bias: Vec3::new(0., 4.5, 0.),
            translation_speed: 3.,
            rotation_speed: 8.
        }
    );
}

// ---

#[allow(dead_code)]
fn follow (
    focus_q: Single<&Transform , With<Body>>,
    cam_q: Single<&mut Transform, (With<Cam>, Without<Body>)>,
    cam_param: Res<CamFollowParams>,
    time: Res<Time>,
) {

    let focus_t = focus_q.into_inner();
    let mut cam_t = cam_q.into_inner();

    let desired = focus_t.translation  +  focus_t.rotation.mul_vec3(cam_param.tranlation_bias);
    cam_t.translation = cam_t.translation.lerp(desired, time.delta_secs() * cam_param.translation_speed);
    let look_at = focus_t.translation+ focus_t.rotation.mul_vec3(cam_param.look_bias);
    cam_t.rotation = cam_t.rotation.slerp(cam_t.looking_at(look_at, Vec3::Y).rotation, time.delta_secs() * cam_param.rotation_speed);
}
