mod collisions;
mod end_turn;
mod entity_renders;
mod map_renders;
mod movement;
mod player_input;
mod random_movement;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renders::map_renders_system())
        .add_system(entity_renders::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_renders::map_renders_system())
        .add_system(entity_renders::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_movement::random_move_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_renders::map_renders_system())
        .add_system(entity_renders::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
