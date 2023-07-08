#![allow(clippy::module_name_repetitions)]
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::bundles::{Name, PhysicsInteractableBundle};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_world);
    }
}

fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::ANTIQUE_WHITE.into());

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            mesh: meshes.add(shape::Box::new(100.0, 1.0, 100.0).into()),
            material,
            ..Default::default()
        },
        Collider::cuboid(50.0, 0.5, 50.0),
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 6000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PhysicsInteractableBundle {
        pbr_bundle: MaterialMeshBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 2.0, 2.0),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        },
        collider: Collider::cuboid(0.5, 0.5, 0.5),

        rigidbody: RigidBody::Dynamic,
        friction: Friction::new(2.0),
        name: Name("Test cube".into()),
        ..Default::default()
    });
}
