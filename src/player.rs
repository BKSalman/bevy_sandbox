use bevy::prelude::*;
use crate::{sprite::{SpriteSheet, spawn_sprite}, TILE_SIZE};
use bevy_inspector_egui::Inspectable;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed:f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    // let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    // let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
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
        .insert(Player { speed: 15.0 });

}