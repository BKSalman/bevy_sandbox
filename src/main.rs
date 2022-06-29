use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_rapier2d::{
    prelude::*,
    // rapier::prelude::ColliderHandle
    };

mod sprite;
mod player;
mod debug;
mod letter_blocks;
mod tile_map;

use sprite::SpritePlugin;
use player::{PlayerPlugin, Player};
// use debug::DebugPlugin;
use letter_blocks::{LettersPlugin, Letter};
// use tile_map::TileMapPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    let height = 640.0;
    App::new()
    .insert_resource(WindowDescriptor {
        width: height * RESOLUTION,
        height: height,
        title: "Letters".to_string(),
        present_mode: PresentMode::Fifo,
        #[cfg(target_arch = "wasm32")]
        canvas: Some("#bevy-canvas".to_string()),
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    // .add_plugin(DebugPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.1))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(SpritePlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(LettersPlugin)
    // .add_plugin(TileMapPlugin)
    .add_startup_system(spawn_camera)
    .add_system(display_event)
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

fn display_event(
    events_query: Query<&Letter, Without<Player>>
) {
    for letter in events_query.iter() {
        println!("{:?}",&letter);
    }
}

// fn display_events(
//     mut collision_events: EventReader<CollisionEvent>,
// ) {
//     for collision_event in collision_events.iter() {
//         match collision_event.to_owned() {
//             CollisionEvent::Started(entity1, entity2, _) => {
//                 println!("Started:: entity1: {:?} entity2: {:?}", entity1, entity2);
//             }
//             CollisionEvent::Stopped(entity1, entity2, _) => {
//                 println!("Stopped:: entity1: {:?} entity2: {:?}", entity1, entity2);
//             }
//         }
//     }
// }
