use bevy::{
    prelude::*
};
use avian3d::{math::PI, prelude::*};
pub struct HeroPlugin;
impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, show_gizmos)
        ;
    }
}

// ---

#[derive(Component)]
struct Gyro_V;
#[derive(Component)]
struct Gyro_H;

#[derive(Component)]
pub struct Body;
#[derive(Component)]
struct Wheel;

// ---


fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>
) {
    let black_material = materials.add(Color::BLACK);
    let color_material = materials.add(StandardMaterial {
        // base_color_texture: Some(assets.load("textures/strips.png")),
        base_color: Color::srgba(1., 1., 1., 0.1),
        alpha_mode: AlphaMode::Blend,

        reflectance: 0.1,
        metallic: 0.5,
        ..default()
    });




    // let base_position = Vec3::new(0.,10.,40.);
    let base_position = Vec3::new(0.,1.,0.);

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
            // Friction::new(10.).with_combine_rule(CoefficientCombine::Max)
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
            // Visibility::Hidden

        ))
        .id();

    // let body2_id  = cmd
    //     .spawn((
    //         Mesh3d(meshes.add(Cylinder::new(0.2, 2.))),
    //         MeshMaterial3d(black_material.clone()),
    //         MassPropertiesBundle::from_shape(&Collider::cylinder(0.2, 2.), 40.),
    //         Transform::from_translation(body_position)
    //             .with_rotation(Quat::from_rotation_z(90f32.to_radians()))
    //         ,
    //         // RigidBody::Static,
    //         RigidBody::Dynamic,
    //     ))
    //     .id();

        // let rotated_child = cmd.spawn((
        //     Mesh3d(meshes.add(Cylinder::new(0.2, 2.))),
        //     MeshMaterial3d(black_material.clone()),
        //     MassPropertiesBundle::from_shape(&Collider::cylinder(0.2, 2.), 40.),
        //     Transform::IDENTITY.with_rotation(Quat::from_rotation_z(90f32.to_radians()))
        // )).id();

        // cmd.entity(body_id).add_child(rotated_child);

    // ---

    let gyro_v_dim = (2., 0.4);
    let gyro_v_mesh = meshes.add(Cylinder::new(gyro_v_dim.0, gyro_v_dim.1));
    let gyro_v_position = body_position + Vec3::Y * body_dim.1 * 0.5 ;
    let gyro_v = cmd
        .spawn((
            Mesh3d(gyro_v_mesh.clone()),
            MeshMaterial3d(color_material.clone()),
            Collider::cylinder(gyro_v_dim.0, gyro_v_dim.1),
            // MassPropertiesBundle::from_shape(&Collider::cylinder(2., 0.4), 40.),
            Transform::from_translation(gyro_v_position),
            RigidBody::Dynamic,
            // RigidBody::Static,
            ColliderDensity(15.),
            AngularVelocity(Vec3::Y * 2. * PI * 20.),
            Gyro_V,
            // Visibility::Hidden
        ))
        .id();

    // ---

    let gyro_h_dim = (1.5, 0.2);
    let gyro_h_mesh = meshes.add(Cylinder::new(gyro_h_dim.0, gyro_h_dim.1));
    let gyro_h_position = body_position;

    let gyro_h = cmd
        .spawn((
            Mesh3d(gyro_h_mesh.clone()),
            MeshMaterial3d(color_material.clone()),
            Collider::cylinder(gyro_h_dim.0, gyro_h_dim.1),
            // MassPropertiesBundle::from_shape(&Collider::cylinder(2., 0.4), 40.),
            Transform::from_translation(gyro_h_position )
                .with_rotation(Quat::from_rotation_z(90f32.to_radians()))
            ,
            RigidBody::Dynamic,
            // RigidBody::Static,
            ColliderDensity(5.),
            AngularVelocity(-Vec3::X * 20.),
            Gyro_H,
            // Visibility::Hidden
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
        RevoluteJoint::new(gyro_v, body_id)
            .with_hinge_axis(Vec3::Y)
            .with_local_anchor2(Vec3::Y * body_dim.1 * 0.5)
            .with_point_compliance(0.),
    );

    // cmd.spawn(
    //     RevoluteJoint::new(gyro_h, body_id)
    //         .with_hinge_axis(Vec3::Y)
    //         .with_point_compliance(0.),
    // );


    cmd.spawn(
        RevoluteJoint::new(gyro_h, body_id)
            .with_hinge_axis(Vec3::X)
            .with_local_basis1(Quat::from_rotation_z(90f32.to_radians()))

    );

    // cmd.spawn(FixedJoint::new(body_id, body2_id));
    // cmd.spawn(FixedJoint::new(rotated_child, gyro_h));
    // cmd.spawn(
    //     RevoluteJoint::new(gyro_h, rotated_child)
    //         .with_hinge_axis(Vec3::X)
    //         .with_point_compliance(0.),

    // );







}

// fn show_gizmos(
//     mut gizmos: Gizmos,
//     q: Single<(&Position, &LinearVelocity), With<Body>>
// ) {
//     let (p,v) = q.into_inner();
//     gizmos.ray(p.0, -v.0.normalize() * 10., Color::WHITE);
// }

fn show_gizmos(
    mut gizmos: Gizmos,
    q: Single<&Transform, With<Body>>
) {
    let t = q.into_inner();
    gizmos.ray(t.translation, t.forward() * 10., Color::WHITE);
}
