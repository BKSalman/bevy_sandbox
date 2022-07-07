pub mod assets;
pub mod debug;
pub mod events;
pub mod inventory;
pub mod items;
pub mod letter_blocks;
pub mod player;
pub mod tile_map;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Playing,
}
