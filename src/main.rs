use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

use bevy_rapier2d::prelude::*;

mod sprite;
mod player;
mod debug;
mod letter_blocks;

use sprite::SpritePlugin;
use player::PlayerPlugin;
use debug::DebugPlugin;
use letter_blocks::LettersPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    let height = 640.0;
    App::new()
    .insert_resource(WindowDescriptor {
        width: height * RESOLUTION,
        height: height,
        title: "Bevy Tutorial".to_string(),
        present_mode: PresentMode::Fifo,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(SpritePlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(LettersPlugin)
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