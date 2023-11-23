use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    ecs: &SubWorld,
    entity: &Entity,
    move_intention: &MoveIntention,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(move_intention.destination) {
        commands.add_component(move_intention.entity, move_intention.destination);
    }

    if ecs
        .entry_ref(move_intention.entity)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
    {
        camera.on_player_move(move_intention.destination);
    }

    commands.remove(*entity);
}
