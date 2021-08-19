use crate::*;

pub struct World {
    entity_manager: EntityManager,
    systems: containers::standard::Vec<Box<dyn SystemExec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            systems: containers::standard::Vec::default(),
        }
    }

    pub fn initialize(&mut self) {
        for obj in reflection::database::iterate_struct() {
            println!("{}", obj.get_typename());
            let var = obj.create_instance();
            if var.can_cast_trait::<dyn System>() {
                self.systems.push(var.cast_trait::<dyn System>().unwrap().prepare());
            }
        }
    }

    pub fn update(&mut self) {
        for system in self.systems.iter_mut() {
            system.exec();
        }
    }

    pub fn get_entity_manager(&self) -> &EntityManager {
        &self.entity_manager
    }

    pub fn get_entity_manager_mut(&mut self) -> &mut EntityManager {
        &mut self.entity_manager
    }
}
