use crate::{
    plugins::sprite::{spawn_sprite, SpriteSheet},
    TILE_SIZE,
    // letter_blocks::TileCollider,
};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use super::inventory::Inventory;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    direction: Direction,
    velocity: f32,
    is_moving: bool,
}

#[derive(Inspectable, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
            velocity: 3.0,
            is_moving: false,
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));
    }
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
) {
    for (mut player, mut rb_vels) in player_query.iter_mut() {
        let up = if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            player.direction = Direction::Up;
            player.is_moving = true;
            true
        } else {
            false
        };
        let down = if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            player.direction = Direction::Down;
            player.is_moving = true;
            true
        } else {
            false
        };
        let left = if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            player.direction = Direction::Left;
            player.is_moving = true;
            true
        } else {
            false
        };
        let right = if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            player.direction = Direction::Right;
            player.is_moving = true;
            true
        } else {
            false
        };
        if !up && !down && !right && !left {
            player.is_moving = false;
        }
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.velocity;
    }
}

pub fn spawn_player(mut commands: Commands, sprite: Res<SpriteSheet>) {
    let player = spawn_sprite(
        &mut commands,
        &sprite,
        0,
        Vec3::new(0.0 * TILE_SIZE, -0.0 * TILE_SIZE, 900.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            velocity: 1.0,
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(0.1, -0.1, 900.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.05, 0.05))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::new(0.0))
        .insert(Velocity::linear(Vec2::new(0.0, 0.0)))
        .insert(Inventory::default());
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
