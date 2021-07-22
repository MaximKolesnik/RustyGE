use containers::slots::*;
use containers::standard::*;

#[derive(Copy, Clone)]
pub struct Entity {
    slot: Slot,
}

impl Entity {
    fn new(slot: Slot) -> Self {
        Self {
            slot,
        }
    }

    pub fn get_slot(&self) -> &Slot {
        &self.slot
    }
}

pub struct EntityManager {
    generator: Generator,
    entities_to_destroy: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            entities_to_destroy: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        Entity::new(self.generator.acquire())
    }

    pub fn is_alive(&self, entity: &Entity) -> bool {
        self.generator.is_valid(entity.get_slot())
    }

    pub fn destroy_entity(&mut self, entity: &Entity) {
        self.entities_to_destroy.push(*entity)
    }
}
