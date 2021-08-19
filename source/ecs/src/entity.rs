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
    component_pools: HashMap<std::any::TypeId, Box<dyn crate::ComponentStorageAccessor>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            entities_to_destroy: Vec::new(),
            component_pools: HashMap::default(),
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

    pub fn assign_component(&mut self, entity: &Entity, var: reflection::Variant) {
        let typeid = var.get_type_id().unwrap();
        if !self.component_pools.contains_key(&typeid) {
            let storage_var = var.cast_trait::<dyn crate::ComponentPrivate>()
                .unwrap().create_storage();
            self.component_pools.insert(typeid,
                storage_var.into_boxed_trait::<dyn crate::ComponentStorageAccessor>().unwrap());
        }

        self.component_pools.get_mut(&typeid).unwrap().assign_component(entity, var);
    }

    pub fn get_component_mut<T>(&mut self, entity: &Entity) -> &mut T
        where T: 'static + crate::Component
    {
        let typeid = std::any::TypeId::of::<T>();
        let pool = self.component_pools.get_mut(&typeid).unwrap();
        pool.as_any().downcast_mut::<crate::ComponentStorage<T>>().unwrap()
            .pool.get_mut(entity.get_slot()).unwrap()
    }

    pub(crate) fn get_enities(&self) -> &Generator {
        &self.generator
    }
}
