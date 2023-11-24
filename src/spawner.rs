use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        0..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MoveRandomly {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (2, "goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "orc".to_string(), to_cp437('o'))
}
