use legion::{component, system, world::SubWorld, IntoQuery};

use crate::components::{GravityImmune, Greenness, Position};

#[system]
#[read_component(Position)]
pub fn debug_print_entities(ecs: &SubWorld) {
    let mut query = <&Position>::query();
    for pos in query.iter(ecs) {
        println!("Entity at {:?}", pos);
    }
}

#[system]
#[read_component(Greenness)]
pub fn debug_print_greenness(ecs: &SubWorld) {
    let mut query = <&Greenness>::query();
    for greenness in query.iter(ecs) {
        println!("Entity is {:?}% green", greenness.amount);
    }
}

#[system]
#[read_component(GravityImmune)]
#[write_component(Position)]
pub fn apply_gravity(ecs: &mut SubWorld) {
    let mut query = <&mut Position>::query().filter(!component::<GravityImmune>());
    for pos in query.iter_mut(ecs) {
        pos.y += 1.0;
    }
}

#[system]
#[write_component(Greenness)]
pub fn become_more_green(ecs: &mut SubWorld) {
    let mut query = <&mut Greenness>::query();
    for greenness in query.iter_mut(ecs) {
        greenness.amount += 10;
    }
}
