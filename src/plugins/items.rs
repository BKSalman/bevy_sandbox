use bevy::prelude::*;
use bevy_inspector_egui::{
    Inspectable,
    RegisterInspectable
};
use serde::Deserialize;

use bevy_rapier2d::prelude::*;

use crate::{
    plugins::sprite::{spawn_sprite, SpriteSheet},
    TILE_SIZE,
};

use super::{player::Player, inventory::{Inventory}};

pub struct ItemsPlugin;

#[derive(Component, Inspectable)]
pub struct Pickupable {
    pub(crate) item: ItemType,
}

#[derive(Debug, Inspectable, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum ItemType {
    None,
    Stick,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::None
    }
}

/// The core enum of the game, lists everything that can be held or placed in the game
#[derive(Debug, Inspectable, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum WorldObject {
    Item(ItemType),
}

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_test_objects)
        .add_system(pickup_item)
        .register_inspectable::<WorldObject>()
        .register_inspectable::<Pickupable>();
    }
}

/// Creates our testing map
#[allow(clippy::vec_init_then_push)]
fn spawn_test_objects(mut commands: Commands, sprite: Res<SpriteSheet>) {
    let mut children = Vec::new();

    let item = spawn_sprite(
        &mut commands,
        &sprite,
        15,
        Vec3::new(10.0 * TILE_SIZE, -10.0 * TILE_SIZE, 100.0),
    );
    commands.entity(item)
    .insert(Pickupable{
        item: ItemType::Stick
    })
    .insert(Collider::cuboid(0.1, 0.1))
    .insert(Sensor(true));

    children.push(item);

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Name::new("Test Objects"))
        .push_children(&children);
}

fn pickup_item(
    mut player_query: Query<(&Player, &mut Inventory, Entity)>,
    pickupable_query: Query<(&Pickupable, Entity), Without<Player>>,
    rapier_context: Res<RapierContext>,
    keyboard: Res<Input<KeyCode>>
) {
    let (player, mut inventory, player_e) = player_query.single_mut();
    let (pickup, pickupable_e) = pickupable_query.single();
    if keyboard.just_pressed(KeyCode::Space) {
        if rapier_context.intersection_pair(player_e, pickupable_e) == Some(true) {
            let pickup_and_count = ItemAndCount {
                item: pickup.item,
                count: 1,
            };
            inventory.add(&pickup_and_count);
            // println!("The entities {:?} and {:?} have intersecting colliders!", player_e, pickupable_e);
        }
    }
}

#[derive(Clone, Copy, Default, Inspectable, Deserialize, Debug, PartialEq)]
pub struct ItemAndCount {
    pub item: ItemType,
    pub count: usize,
}

impl std::fmt::Display for ItemAndCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {:?}", self.count, self.item)
    }
}