use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_fps_controller::controller::{
    FpsController, FpsControllerInput, LogicalPlayer, RenderPlayer,
};
use bevy_rapier3d::prelude::*;

#[derive(Default)]
pub struct PlayerPlugin {
    pub player_position: Vec3,
}

#[derive(Resource)]
struct InitialPlayerPosition {
    position: Vec3,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(InitialPlayerPosition {
            position: self.player_position,
        })
        .add_startup_system(spawn_player);
    }
}

fn spawn_player(
    pos: Res<InitialPlayerPosition>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Collider::capsule_y(1.0, 0.25),
        Friction {
            coefficient: 0.0,
            combine_rule: bevy_rapier3d::prelude::CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: bevy_rapier3d::prelude::CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        Ccd { enabled: true },
        TransformBundle::from_transform(Transform::from_translation(pos.position)),
        LogicalPlayer { player_id: 0 },
        FpsControllerInput::default(),
        FpsController::default(),
    ));
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 4.0,
                ..Default::default()
            }),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::rgb(0.7, 0.8, 0.98),
                ),
                ..Default::default()
            },
            ..Default::default()
        },
        RenderPlayer { player_id: 0 },
    ));

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            mesh: meshes.add(shape::Box::new(100.0, 1.0, 100.0).into()),
            material: materials.add(Color::ANTIQUE_WHITE.into()),
            ..Default::default()
        },
        Collider::cuboid(50.0, 0.5, 50.0),
    ));
}
