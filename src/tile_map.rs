use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    sprite::{spawn_sprite, SpriteSheet},
    TILE_SIZE,
};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands, sprite: Res<SpriteSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    let tile:Entity = spawn_sprite(
                        &mut commands,
                        &sprite,
                        15,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    );
                    commands.entity(tile)
                    .insert(RigidBody::Fixed)
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(Collider::cuboid(0.05, 0.05))
                    .insert(GravityScale(0.0));
                    tiles.push(tile);
                }
            }
        }
    }
    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}