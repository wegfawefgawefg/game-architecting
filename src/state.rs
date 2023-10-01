use legion::{IntoQuery, Resources, Schedule, World};

use crate::systems::{
    apply_gravity_system, become_more_green_system, debug_print_entities_system,
    debug_print_greenness_system,
};

pub struct State {
    pub ecs: World,
    pub resources: Resources,
    pub schedule: Schedule,
}

impl State {
    pub fn new() -> State {
        let ecs = World::default();
        let resources = Resources::default();

        // debug_print_entities(&ecs);
        // debug_print_greenness(&ecs);

        // apply_gravity(&mut ecs);
        // become_more_green(&mut ecs);

        // debug_print_greenness(&ecs);
        // debug_print_entities(&ecs);
        let schedule = Schedule::builder()
            .add_system(debug_print_entities_system())
            .add_system(debug_print_greenness_system())
            .add_system(apply_gravity_system())
            .add_system(become_more_green_system())
            .build();

        State {
            ecs,
            resources,
            schedule,
        }
    }
}
