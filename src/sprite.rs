use bevy::prelude::*;
use crate::TILE_SIZE;
pub struct SpritePlugin;

pub struct SpriteSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Sprite;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_sprite);
    }
}

pub fn spawn_sprite(
    commands: &mut Commands,
    sprite: &SpriteSheet,
    index: usize,
    translation: Vec3,
) -> Entity {

    let mut sprite1 = TextureAtlasSprite::new(index);
    sprite1.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite1,
            texture_atlas: sprite.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn load_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(texture_handle,
        Vec2::splat(15.0),
        14,
        1,
        Vec2::splat(2.));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheet(texture_atlas_handle));
}