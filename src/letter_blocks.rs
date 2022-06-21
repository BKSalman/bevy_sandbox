use bevy::prelude::*;
use crate::{sprite::{SpriteSheet, spawn_sprite}, TILE_SIZE, RESOLUTION};
use rand::prelude::*;

#[derive(Component)]
pub struct Letter;

pub struct LettersPlugin;

impl Plugin for LettersPlugin {
    fn build (&self, app: &mut App) {
        // app.add_startup_system(spawn_letter_blocks);
        app.add_system(spawn_letter_blocks);
    }
}

enum ArabicLetters {
    None,
    Alif,
    Baa,
    Taa,
    Thaa,
    Jeem,
    Haa,
    Khaa,
    Daal,
    Thaal
}

pub fn spawn_letter_blocks(
    mut commands: Commands,
    sprite: Res<SpriteSheet>,
    keyboard: Res<Input<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let block = spawn_sprite(
            &mut commands,
            &sprite,
            ArabicLetters::Alif as usize,
            Vec3::new(rand::thread_rng().gen_range(0.0..1.0) * TILE_SIZE, rand::thread_rng().gen_range(-(1.0 * RESOLUTION)..0.0) * TILE_SIZE, 900.0),
        );
        
    
        commands
            .entity(block)
            .insert(Name::new("Letter"))
            .insert(Letter);
    }
}