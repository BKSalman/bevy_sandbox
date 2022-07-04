use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use super::items::{
    ItemAndCount, 
    // Pickupable, 
    ItemType
};
// use bevy_inspector_egui::{
//     Inspectable,
//     RegisterInspectable
// };


const INVENTORY_SIZE: usize = 5;

const INVENTORY_ITEM_SIZE: usize = 5;
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_inspectable::<Inventory>();
    }
}

pub struct InventoryOverflow(pub usize);

#[derive(Component, Default, Inspectable, Clone)]
pub struct Inventory {
    pub items: [ItemAndCount; INVENTORY_SIZE]
}

impl Inventory {
    pub fn add(&mut self, item_and_count: &ItemAndCount) -> Option<InventoryOverflow> {
        let mut remaining_amount = item_and_count.count;

        for item in self
            .items
            .iter_mut()
            .filter(|item| item.item != ItemType::None)
        {
            if item.item == item_and_count.item {
                let addable_item_count =
                    std::cmp::min(remaining_amount, INVENTORY_ITEM_SIZE - item_and_count.count);
                item.count += addable_item_count;
                remaining_amount -= addable_item_count;
                if remaining_amount == 0 {
                    return None;
                }
            }
        }

        for item in self
            .items
            .iter_mut()
            .filter(|item| item.item == ItemType::None)
        {
            item.item = item_and_count.item;
            let addable_item_count =
                std::cmp::min(remaining_amount, INVENTORY_ITEM_SIZE - item_and_count.count);
            item.count = addable_item_count;
            remaining_amount -= item.count;
            if remaining_amount == 0 {
                return None;
            }
        }
        Some(InventoryOverflow(remaining_amount))
    }
        
}

