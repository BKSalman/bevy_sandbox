// use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy_asset_loader::*;

use crate::TILE_SIZE;

pub struct GameAssetsPlugin;

#[derive(AssetCollection)]
pub struct SpriteSheet {
    #[asset(texture_atlas(
        tile_size_x = 15.,
        tile_size_y = 15.,
        columns = 14,
        rows = 2,
        padding_x = 2.,
        padding_y = 2.
    ))]
    #[asset(path = "sprite_sheet.png")]
    pub sprite_sheet: Handle<TextureAtlas>,
}

pub struct Graphics {
    pub texture_atlas: Handle<TextureAtlas>,
    // maybe add commonly used sprite indices
}

#[derive(Component)]
pub struct Sprite;

impl Plugin for GameAssetsPlugin {
    fn build(&self, _app: &mut App) {
        // app
        // .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(draw_atlas));
        // .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite);
    }
}

pub fn spawn_sprite(
    commands: &mut Commands,
    sprite: Handle<TextureAtlas>,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite_index = TextureAtlasSprite::new(index);
    sprite_index.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite_index,
            texture_atlas: sprite,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
