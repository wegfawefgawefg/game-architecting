#![warn(clippy::pedantic)]

use components::{Calcium, GravityImmune, Greenness, Position};
use legion::{IntoQuery, World};
use state::State;

mod components;
mod state;
mod systems;

fn main() {
    let mut state = State::new();

    // goblin has greenness
    state.ecs.push((
        Position { x: 0.0, y: 0.0 },
        Calcium { amount: 0 },
        Greenness { amount: 0 },
    ));
    //  skeleton has no greenness, but has calcium
    state
        .ecs
        .push((Position { x: 0.0, y: 0.0 }, Calcium { amount: 0 }));

    // state.ecs.extend(vec![
    //     // add a bird
    //     (Position { x: 2.0, y: 2.0 }, GravityAffected(false)),
    //     // add a bat
    //     (Position { x: 3.0, y: 3.0 }, GravityAffected(false)),
    // ]);

    // execute the systems here
    loop {
        state.schedule.execute(&mut state.ecs, &mut state.resources);
    }
}
