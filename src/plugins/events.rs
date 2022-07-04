use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::player::{
    Player
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(test_events);
    }
}

// use super::letter_blocks::Letter;

fn test_events(
    player_query: Query<(Entity, &Player)>,
    // letters_query: Query<(Entity, &Letter)>,
    rapier_context: Res<RapierContext>
) {
    let (e, _) = player_query.single();
    // let Letters: Vec<Entity, &Letter> = letters_query.iter().collect();

    for contact_with in rapier_context.contacts_with(e) {
        let other_collider = if contact_with.collider1() == e {
            contact_with.collider2()
        } else {
            contact_with.collider1()
        };

        println!("{:?}", other_collider);
    }

}
