use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// use bevy_ecs_tilemap::prelude::*;

use crate::{
    plugins::sprite::{spawn_sprite, SpriteSheet},
    TILE_SIZE,
};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
        // app.add_startup_system(create_map);
    }
}

fn create_simple_map(mut commands: Commands, sprite: Res<SpriteSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();
    let mut shapes: Vec<(Vec2, f32, Collider)> = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let position = Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0);
                if char == '#' {
                    let tile:Entity = spawn_sprite(
                        &mut commands,
                        &sprite,
                        x+1,
                        position,
                    );
                    commands.entity(tile)
                    .insert(RigidBody::Fixed)
                    .insert(LockedAxes::ROTATION_LOCKED);
                    // .insert(GravityScale(0.0))
                    tiles.push(tile);
                    shapes.push((Vec2::new(position.x, position.y), 0.0, Collider::round_cuboid(0.01, 0.01, 0.4)));
                }
            }
        }
    }
    
    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Collider::compound(shapes))
        .push_children(&tiles);
}

// fn create_map (
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut map_query: MapQuery,
// ) {
//     asset_server.watch_for_changes().unwrap();
//     let handle: Handle<LdtkMap> = asset_server.load("map.ldtk");

//     let map_entity = commands.spawn().id();

//     commands.entity(map_entity).insert_bundle(LdtkMapBundle {
//         ldtk_map: handle,
//         map: Map::new(0u16, map_entity),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..Default::default()
//     });
// }