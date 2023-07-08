#![allow(clippy::needless_pass_by_value)]
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use player::PlayerPlugin;
use world::WorldPlugin;

mod bundles;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FpsControllerPlugin)
        .add_plugin(PlayerPlugin {
            player_position: Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
        })
        .add_plugin(WorldPlugin)
        .add_system(cursor_manager)
        .run();
}

fn cursor_manager(
    mut window: Query<&mut Window>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut window = window.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
