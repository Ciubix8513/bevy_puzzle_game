use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct Interactable;

#[derive(Component, Default)]
pub struct Name(pub String);

#[derive(Bundle, Default)]
pub struct PhysicsInteractableBundle {
    pub rigidbody: RigidBody,
    pub pbr_bundle: PbrBundle,
    pub interactable: Interactable,
    pub friction: Friction,
    pub collider: Collider,
    pub name: Name,
}
