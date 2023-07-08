#![allow(clippy::module_name_repetitions)]
use std::f32::consts::TAU;

use bevy::{math::Vec3A, prelude::*};
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
        .add_startup_system(spawn_player)
        .add_system(player_raycast);
    }
}

fn player_raycast(
    context: Res<RapierContext>,
    player: Query<&Transform, With<RenderPlayer>>,
    player_collider: Query<Entity, With<LogicalPlayer>>,
    input: Res<Input<KeyCode>>,
    world: &World,
) {
    //Do nothing unless just pressed E
    if !input.just_pressed(KeyCode::E) {
        return;
    }

    let player = player.single();
    let rotation = player.rotation;
    //Is forward = -Z wtf?
    let dir = rotation.mul_vec3a(Vec3A::NEG_Z);

    let obj = context.cast_ray(
        player.translation.into(),
        dir.into(),
        10.0,
        false,
        QueryFilter::new().exclude_collider(player_collider.single()),
    );

    if obj.is_none() {
        return;
    }

    let obj = obj.unwrap().0;
    match world.get::<crate::bundles::Name>(obj) {
        Some(name) => info!("Name is {}", name.0),
        None => info!("No name"),
    }
}

fn spawn_player(pos: Res<InitialPlayerPosition>, mut commands: Commands) {
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
}
