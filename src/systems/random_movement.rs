use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MoveRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MoveRandomly)>::query();
    let mut layout = <(Entity, &Point, &Health)>::query();

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();

        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        let mut attacked = false;

        layout
            .iter(ecs)
            .filter(|(_, pos, _)| **pos == destination)
            .for_each(|(defender, _, _)| {
                if ecs
                    .entry_ref(*defender)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        AttackIntention {
                            attacker: *entity,
                            defender: *defender,
                        },
                    ));

                    attacked = true;
                }
            });

        if !attacked {
            commands.push((
                (),
                MoveIntention {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
