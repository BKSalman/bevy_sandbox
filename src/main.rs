// use std::env;
use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_rapier2d::{
    prelude::*,
    // rapier::prelude::ColliderHandle
    };

// use bevy_ecs_tilemap::prelude::*;

mod plugins;

use plugins::sprite::SpritePlugin;
use plugins::player::{
    PlayerPlugin,
    // Player
};
use plugins::debug::DebugPlugin;
use plugins::letter_blocks::{
    LettersPlugin
};
// use plugins::tile_map::TileMapPlugin;
// use plugins::items::ItemsPlugin;
// use plugins::inventory::InventoryPlugin;
// use plugins::events::EventsPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let height = 640.0;
    App::new()
    .insert_resource(WindowDescriptor {
        width: height * RESOLUTION,
        height,
        position: Some(Vec2::new(400.0, 200.0)),
        title: "Letters".into(),
        present_mode: PresentMode::Fifo,
        #[cfg(target_arch = "wasm32")]
        canvas: Some("#bevy-canvas".to_string()),
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.1))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(SpritePlugin)
    .add_plugin(PlayerPlugin)
    // .add_plugin(InventoryPlugin)
    // .add_plugin(ItemsPlugin)
    .add_plugin(LettersPlugin)
    // .add_plugin(EventsPlugin)
    // .add_plugin(TileMapPlugin)
    .insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    })
    .add_startup_system(spawn_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    //Set the camera to have normalized coordinates of y values -1 to 1
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    //Force the camera to use our settings
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
