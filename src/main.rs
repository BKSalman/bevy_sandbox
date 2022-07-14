// use std::env;
use bevy::{
    prelude::*, 
    window::PresentMode
};
use bevy_asset_loader::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

mod plugins;
mod systems;

use plugins::assets::{MyAssets};
use plugins::debug::DebugPlugin;
use plugins::tile_map::{TileMapPlugin, WallBundle, Map};
// use plugins::letter_blocks::LettersPlugin;
use plugins::player::{PlayerPlugin, PlayerBundle};
use plugins::GameState;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;
pub const HEIGHT: f32 = 640.0;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_loopless_state(GameState::AssetLoading);

    AssetLoader::new(GameState::AssetLoading)
        // https://github.com/NiklasEi/bevy_asset_loader/issues/54
        .continue_to_state(GameState::Playing)
        .with_collection::<MyAssets>()
        .with_collection::<Map>()
        .build(&mut app);

        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(systems::pause_physics_during_load))
        app.insert_resource(WindowDescriptor {
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
        .add_plugin(ProgressPlugin::new(GameState::AssetLoading))
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(DebugPlugin)
        .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .add_enter_system(GameState::Playing, setup)
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, map: Res<Map>) {
    let camera = OrthographicCameraBundle::new_2d();
    
    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: map.map.clone(),
        ..Default::default()
    });
}