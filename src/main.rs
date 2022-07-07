// use std::env;
use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_asset_loader::*;
// use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod plugins;

use plugins::assets::{GameAssetsPlugin, SpriteSheet};
use plugins::debug::DebugPlugin;
use plugins::letter_blocks::LettersPlugin;
use plugins::player::PlayerPlugin;
use plugins::GameState;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;
pub const HEIGHT: f32 = 640.0;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();

    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::Playing)
        .with_collection::<SpriteSheet>()
        .build(&mut app);

    app.add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera))
        .insert_resource(WindowDescriptor {
            height: HEIGHT,
            width: HEIGHT * RESOLUTION,
            position: Some(Vec2::new(400.0, 200.0)),
            title: "Letters".into(),
            present_mode: PresentMode::Fifo,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy-canvas".to_string()),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameAssetsPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.1))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        // .add_plugin(LdtkPlugin)
        // .insert_resource(LevelSelection::Index(0))
        // .add_plugin(InventoryPlugin)
        // .add_plugin(ItemsPlugin)
        .add_plugin(LettersPlugin)
        // .add_plugin(EventsPlugin)
        // .add_plugin(TileMapPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        // .add_startup_system(spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    // Set the camera to have normalized coordinates of y values -1 to 1
    camera.orthographic_projection.top = 1.;
    camera.orthographic_projection.bottom = -1.;

    camera.orthographic_projection.right = camera.orthographic_projection.top * RESOLUTION;
    camera.orthographic_projection.left = camera.orthographic_projection.bottom * RESOLUTION;

    //Force the camera to use our settings
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
