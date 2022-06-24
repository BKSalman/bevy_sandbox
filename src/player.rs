use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_rapier2d::{
    prelude::*,
    // rapier::{
    //     dynamics::RigidBodyBuilder,
    // }
};
use crate::{
    sprite::{SpriteSheet, spawn_sprite},
    TILE_SIZE,
    // letter_blocks::TileCollider,
};
use bevy_inspector_egui::Inspectable;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    direction:Direction,
    velocity:f32,
    is_moving:bool,
}

#[derive(Inspectable, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
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
        .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&mut Player,&mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
) {
    for (mut player, mut rb_vels) in player_query.iter_mut() {
        let up = if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            player.direction = Direction::Up;
            player.is_moving = true;
            true
        } else {false};
        let down = if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            player.direction = Direction::Down;
            player.is_moving = true;
            true
        } else {false};
        let left = if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left){
            player.direction = Direction::Left;
            player.is_moving = true;
            true
        } else {false};
        let right = if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            player.direction = Direction::Right;
            player.is_moving = true;
            true
        } else {false};
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
        .insert(Player { velocity: 3.0, ..Default::default() })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.05, 0.05))
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity {
            linvel: Vec2::new(0.0,0.0),
            ..Default::default()
        });

}

// fn wall_collision_check(
//     target_player_pos: Vec3,
//     wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
// ) -> bool {
//     for wall_transform in wall_query.iter() {
//         let collision = collide(
//             target_player_pos,
//             Vec2::splat(TILE_SIZE * 0.9),
//             wall_transform.translation,
//             Vec2::splat(TILE_SIZE),
//         );
//         if collision.is_some() {
//             return false;
//         }
//     }
//     true
// }
