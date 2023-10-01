#![warn(clippy::pedantic)]

use legion::{IntoQuery, World};

mod systems;

#[derive(Clone, Copy, Debug, PartialEq)]
struct GravityAffected(bool);

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
    let mut ecs = World::default();

    // goblin has greenness
    ecs.push((
        Position { x: 0.0, y: 0.0 },
        Calcium { amount: 0 },
        Greenness { amount: 0 },
    ));
    //  skeleton has no greenness, but has calcium
    ecs.push((Position { x: 1.0, y: 1.0 }, Calcium { amount: 0 }));

    ecs.extend(vec![
        // add a bird
        (Position { x: 2.0, y: 2.0 }, GravityAffected(false)),
        // add a bat
        (Position { x: 3.0, y: 3.0 }, GravityAffected(false)),
    ]);

    debug_print_entities(&ecs);
    debug_print_greenness(&ecs);

    apply_gravity(&mut ecs);
    become_more_green(&mut ecs);

    debug_print_greenness(&ecs);
    debug_print_entities(&ecs);
}

pub fn debug_print_entities(ecs: &World) {
    let mut query = <&Position>::query();
    for pos in query.iter(ecs) {
        println!("Entity at {:?}", pos);
    }
}

pub fn debug_print_greenness(ecs: &World) {
    let mut query = <&Greenness>::query();
    for greenness in query.iter(ecs) {
        println!("Entity is {:?}% green", greenness.amount);
    }
}

pub fn apply_gravity(ecs: &mut World) {
    let mut query = <(&mut Position, &GravityAffected)>::query();
    for (pos, gravity_affected) in query.iter_mut(ecs) {
        if gravity_affected.0 {
            // Only apply gravity to entities marked as gravity-affected.
            pos.x += 1.0;
            pos.y += 1.0;
        }
    }
}

pub fn become_more_green(ecs: &mut World) {
    let mut query = <&mut Greenness>::query();
    for greenness in query.iter_mut(ecs) {
        greenness.amount += 10;
    }
}
