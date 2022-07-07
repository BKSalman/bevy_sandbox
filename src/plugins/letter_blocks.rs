use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    plugins::assets::{spawn_sprite, SpriteSheet},
    plugins::player::Player,
    RESOLUTION,
};

use super::GameState;

#[derive(Component, Clone, Copy, Debug)]
pub struct Letter;

pub struct LettersPlugin;

impl Plugin for LettersPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(spawn_letter_blocks);
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(spawn_letter_blocks),
        );
    }
}

#[allow(dead_code)]
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
    Thaal,
}

pub fn spawn_letter_blocks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    sprite: Res<SpriteSheet>,
    keyboard: Res<Input<KeyCode>>,
) {
    let transform = player_query.single();
    if keyboard.just_pressed(KeyCode::Space) {
        let block = spawn_sprite(
            &mut commands,
            sprite.sprite_sheet.clone(),
            rand::thread_rng().gen_range(1..26) as usize, // for testing, use ArabicLetters enum later
            Vec3::new(
                transform.translation.x
                    + (rand::thread_rng().gen_range(-(0.9 * RESOLUTION)..(0.9 * RESOLUTION))),
                transform.translation.y + (rand::thread_rng().gen_range(-0.9..0.9)),
                900.0,
            ),
        );

        commands
            .entity(block)
            .insert(Name::new("Letter"))
            .insert(Letter)
            .insert(RigidBody::Dynamic)
            .insert(Damping {
                linear_damping: 50.0,
                ..Default::default()
            })
            .insert(LockedAxes::ROTATION_LOCKED)
            // .insert(Collider::round_cuboid(0.01, 0.01, 0.4))
            .insert(Collider::cuboid(0.05, 0.05))
            .insert(Ccd::enabled())
            .insert(Restitution::new(0.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(GravityScale(0.0));
    }
}
