use legion::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Calcium {
    amount: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Greenness {
    amount: u32,
}

fn main() {
    let mut world = World::default();

    // goblin has greenness
    world.push((
        Position { x: 0.0, y: 0.0 },
        Calcium { amount: 0 },
        Greenness { amount: 0 },
    ));
    //  skeleton has no greenness, but has calcium
    world.push((Position { x: 1.0, y: 1.0 }, Calcium { amount: 0 }));

    debug_print_entities(&world);
    debug_print_greenness(&world);

    apply_gravity(&mut world);
    become_more_green(&mut world);

    debug_print_greenness(&world);
    debug_print_entities(&world);
}

pub fn debug_print_entities(world: &World) {
    let mut query = <Read<Position>>::query();
    for pos in query.iter(world) {
        println!("Entity at {:?}", pos);
    }
}

pub fn debug_print_greenness(world: &World) {
    let mut query = <Read<Greenness>>::query();
    for greenness in query.iter(world) {
        println!("Entity is {:?}% green", greenness.amount);
    }
}

pub fn apply_gravity(world: &mut World) {
    let mut query = <Write<Position>>::query();
    for pos in query.iter_mut(world) {
        pos.x += 1.0;
        pos.y += 1.0;
    }
}

pub fn become_more_green(world: &mut World) {
    let mut query = <Write<Greenness>>::query();
    for greenness in query.iter_mut(world) {
        greenness.amount += 10;
    }
}
