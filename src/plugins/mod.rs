use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use self::player::PlayerBundle;

pub mod assets;
pub mod debug;
// pub mod events;
// pub mod inventory;
// pub mod items;
// pub mod letter_blocks;
pub mod player;
pub mod tile_map;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Playing,
}


#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub gravity_scale: GravityScale,
    pub name: Name,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass_properties: ColliderMassProperties,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                gravity_scale: GravityScale(0.),
                name: Name::new("Player"),
                collider: Collider::cuboid(6., 14.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        if int_grid_cell.value == 2 {
            ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rotation_constraints,
                ..Default::default()
            }
        } else {
            ColliderBundle::default()
        }
    }
}