use bevy::{
    prelude::*,
    mesh::VertexAttributeValues
};
use avian3d::prelude::*;
use noise::{Perlin, BasicMulti, NoiseFn};
pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
        ;
    }
}

// ---

// fn startup(
//     mut cmd: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>
// ) {
//     let mat_black = materials.add(Color::BLACK);
//     let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5., 10.)));
//     let collider = Collider::cuboid(10., 0.01, 20.);
//     cmd.spawn((
//         Mesh3d(mesh.clone()),
//         MeshMaterial3d(mat_black.clone()),        vertex[1] = noise.get([0.1, 0.1]) as f32;
//         Transform::from_xyz(0., 0., 0.)
//             .with_rotation(Quat::from_rotation_x(-5.0_f32.to_radians())),
//         collider.clone(),
//         RigidBody::Static,
//     ));

//     cmd.spawn((
//         Mesh3d(mesh.clone()),
//         MeshMaterial3d(mat_black.clone()),
//         Transform::from_xyz(0., -3., -20.)
//             .with_rotation(Quat::from_rotation_y(-5.0_f32.to_radians()) * Quat::from_rotation_z(-5.0_f32.to_radians()))
//             ,
//         collider,
//         RigidBody::Static,
//     ));


// }


fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mat_black = materials.add(Color::BLACK);
    let mut mesh = Mesh::from(Plane3d::default().mesh().size(50., 200.).subdivisions(100));

    let Some(VertexAttributeValues::Float32x3(verticis)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        return;
    };
    let noise = BasicMulti::<Perlin>::default();
    for vertex in verticis {
        vertex[1] = noise.get([(vertex[0] as f64), (vertex[2] as f64)/ 2. ]) as f32
        // if vertex[2] > 10. {
        //     vertex[1] = vertex[2].powf(1.9) * 0.005 as f32 ;
        // } else if vertex[2] < -40.{
        //     vertex[1] = 0.;

        // } else {
        //     vertex[1] = noise.get([(vertex[0] as f64), (vertex[2] as f64)]) as f32
        //         // * vertex[2].abs() as f32 * 0.05
        //     ;
        // }

    }

    mesh.compute_normals();


    cmd.spawn((
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(mat_black.clone()),
        Transform::from_xyz(0., 0., 0.)
            // .with_rotation(Quat::from_rotation_x(-5.0_f32.to_radians()))
            ,
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
    ));

}
