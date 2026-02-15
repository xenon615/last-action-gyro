use bevy::{
    input::common_conditions::input_just_pressed, prelude::*
};
use avian3d::{math::PI, prelude::*};
pub struct HeroPlugin;
impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, go.run_if(input_just_pressed(KeyCode::Space)))
            .add_systems(Update, stop_rotation.run_if(input_just_pressed(KeyCode::Enter)))
        ;
    }
}

// ---

#[derive(Component)]
struct Gyro;

#[derive(Component)]
pub struct Body;

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>
) {
    let black_material = materials.add(Color::BLACK);
    let color_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.load("textures/strips.png")),
        // base_color: Color::srgba(1., 1., 1., 0.1),
        // alpha_mode: AlphaMode::Blend,
        reflectance: 0.0,
        metallic: 0.5,
        ..default()
    });

    let base_position = Vec3::new(0.,1.,90.);

    // ---

    let wheel_radius = 1.;
    let wheel_position = base_position + Vec3::Y * wheel_radius;
    let wheel_id = cmd
        .spawn((
            Mesh3d(meshes.add(Sphere::new(wheel_radius))),
            MeshMaterial3d(color_material.clone()),
            Transform::from_translation(wheel_position),
            Collider::sphere(wheel_radius),
            RigidBody::Dynamic,
            // RigidBody::Static,
            ColliderDensity(10.),
            Restitution::new(0.).with_combine_rule(CoefficientCombine::Max),
        ))
        .id();

    // ---

    let body_dim = (0.6, 4.);
    let body_mesh = meshes.add(Cylinder::new(body_dim.0, body_dim.1));
    let body_position = wheel_position + Vec3::Y * (body_dim.1 * 0.5 + 0.5 * wheel_radius);
    let body_id  = cmd
        .spawn((
            Mesh3d(body_mesh.clone()),
            MeshMaterial3d(black_material.clone()),
            MassPropertiesBundle::from_shape(&Collider::cylinder(body_dim.0, body_dim.1), 40.),
            Transform::from_translation(body_position),
            // RigidBody::Static,
            RigidBody::Dynamic,
            Body,
            ConstantForce(Vec3::ZERO)
        ))
        .id();

    // ---

    let gyro_dim = (3., 0.4);
    let gyro_mesh = meshes.add(Cylinder::new(gyro_dim.0, gyro_dim.1));
    let gyro_position = body_position + Vec3::Y * body_dim.1 * 0.5 ;
    let gyro = cmd
        .spawn((
            Mesh3d(gyro_mesh.clone()),
            MeshMaterial3d(color_material.clone()),
            Collider::cylinder(gyro_dim.0, gyro_dim.1),
            Transform::from_translation(gyro_position),
            RigidBody::Dynamic,
            // RigidBody::Static,
            ColliderDensity(15.),
            AngularVelocity(Vec3::Y * 2. * PI * 40.),
            Gyro,
        ))
        .id();

    // --- JOINTS ---

    cmd.spawn((
        SphericalJoint::new(body_id, wheel_id)
            .with_local_anchor1(-Vec3::Y * (body_dim.1 * 0.5 + wheel_radius * 0.5))
            .with_point_compliance(0.),
        JointCollisionDisabled
    ))
    ;

    cmd.spawn(
        RevoluteJoint::new(gyro, body_id)
            .with_hinge_axis(Vec3::Y)
            .with_local_anchor2(Vec3::Y * body_dim.1 * 0.5)
            .with_point_compliance(0.),
    );
}

// ---

fn go (
    q: Single<&mut ConstantForce, With<Body>>
) {
    q.into_inner().0 = -Vec3::Z * 800.;
}

// ---

fn stop_rotation (
    q: Single<&mut AngularVelocity, With<Gyro>>
) {
    q.into_inner().0 = Vec3::ZERO;
}
