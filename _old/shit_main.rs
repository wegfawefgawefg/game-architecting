use glam::Vec2;
use std::any::Any;
pub struct Skeleton {
    id: u32,
    pos: Vec2,
    calcium: u32,
}

impl Entity for Skeleton {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    fn as_any(&mut self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Goblin {
    id: u32,
    pos: Vec2,
    greenness: u32,
}

impl Entity for Goblin {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    fn as_any(&mut self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Entity {
    fn get_id(&self) -> u32;
    fn get_pos(&self) -> Vec2;
    fn set_pos(&mut self, pos: Vec2);
    fn as_any(&mut self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub fn debug_print_entities(entities: &Vec<Box<dyn Entity>>) {
    for entity in entities.iter() {
        println!("Entity {:?} at {:?}", entity.get_id(), entity.get_pos());
    }
}

pub fn debug_print_greenness(entities: &[Box<dyn Entity>]) {
    for entity in entities.iter() {
        if let Some(goblin) = entity.as_any().downcast_ref::<Goblin>() {
            // Only Goblins can become more green, so check the type.
            println!(
                "Goblin {:?} is {:?}% green",
                goblin.get_id(),
                goblin.greenness
            );
        }
    }
}
pub fn apply_gravity(entities: &mut [Box<dyn Entity>]) {
    for entity in entities.iter_mut() {
        entity.set_pos(entity.get_pos() + Vec2::new(0.0, -1.0));
    }
}

pub fn become_more_green(entities: &mut [Box<dyn Entity>]) {
    for entity in entities.iter_mut() {
        if let Some(goblin) = entity.as_any().downcast_mut::<Goblin>() {
            // Only Goblins can become more green, so check the type.
            goblin.greenness += 10;
        }
    }
}

fn main() {
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();
    entities.push(Box::new(Skeleton {
        id: 0,
        pos: Vec2::new(0.0, 0.0),
        calcium: 100,
    }));
    entities.push(Box::new(Goblin {
        id: 1,
        pos: Vec2::new(0.0, 0.0),
        greenness: 100,
    }));

    debug_print_entities(&entities);
    debug_print_greenness(&entities);

    apply_gravity(entities.as_mut_slice());
    become_more_green(entities.as_mut_slice());

    debug_print_entities(&entities);
    debug_print_greenness(&entities);
}

// data in common between entities
// unique data, only some entities have

// systems that apply to all entities
// systems that apply to some entities

/*  DOWNSIDES:
    every entitiy has to reimplent getters and setters for all common data
*/
