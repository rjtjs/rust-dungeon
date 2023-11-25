use crate::prelude::*;

#[system]
#[read_component(AttackIntention)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attack_intentions = <(Entity, &AttackIntention)>::query();

    let defenders: Vec<(Entity, Entity)> = attack_intentions
        .iter(ecs)
        .map(|(entity, attack_intention)| (*entity, attack_intention.defender))
        .collect();

    defenders.iter().for_each(|(message, defender)| {
        if let Ok(health) = ecs
            .entry_mut(*defender)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;

            if health.current < 1 {
                commands.remove(*defender);
            }
        }

        commands.remove(*message);
    });
}
