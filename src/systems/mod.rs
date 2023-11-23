mod collisions;
mod entity_renders;
mod map_renders;
mod player_input;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .add_system(map_renders::map_renders_system())
        .add_system(entity_renders::entity_render_system())
        .build()
}
