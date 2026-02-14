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

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mat_black = materials.add(Color::BLACK);
    let mut mesh = Mesh::from(Plane3d::default().mesh().size(100., 200.).subdivisions(100));

    let Some(VertexAttributeValues::Float32x3(verticis)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        return;
    };
    let noise = BasicMulti::<Perlin>::default();
    for vertex in verticis {
        vertex[1] = noise.get([(vertex[0] as f64), (vertex[2] as f64)/ 2. ]) as f32
    }

    mesh.compute_normals();
    cmd.spawn((
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(mat_black.clone()),
        Transform::from_xyz(0., 0., 0.),
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
    ));
}
