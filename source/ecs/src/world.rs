use crate::entity::*;

struct World {
    entity_manager: EntityManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
        }
    }

    pub fn get_entity_manager(&self) -> &EntityManager {
        &self.entity_manager
    }

    pub fn get_entity_manager_mut(&mut self) -> &mut EntityManager {
        &mut self.entity_manager
    }
}
