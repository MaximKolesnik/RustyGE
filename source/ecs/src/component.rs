use containers::*;
use reflection::Reflected;
use crate::*;

pub trait Component {
}

pub(crate) trait ComponentPrivate {
    fn create_storage(&self) -> reflection::Variant;
}

impl<T: 'static + Component + reflection::Reflected> ComponentPrivate for T {
    fn create_storage(&self) -> reflection::Variant
        where ComponentStorage<T>: Reflected {
        ComponentStorage::<T>::create_variant()
    }
}

#[reflect]
pub(crate) struct ComponentStorage<T: 'static + Component> {
    pub pool: slots::SparseStorage<T>,
}

impl<T: 'static + Component> Default for ComponentStorage<T> {
    fn default() -> Self {
        Self {
            pool: slots::SparseStorage::new(),
        }
    }
}

pub(crate) trait ComponentStorageAccessor {
    fn as_any(&mut self) -> &mut dyn std::any::Any;
    fn assign_component(&mut self, entity: &Entity, var: reflection::Variant);
}

impl<T: 'static + Component> ComponentStorageAccessor for ComponentStorage<T> {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn assign_component(&mut self, entity: &Entity, var: reflection::Variant) {
        let val = var.into_raw_object::<T>();
        self.pool.create(entity.get_slot(), val.expect("Was not able to extact component"));
    }
}

// pub struct ComponentManager {
//     component_pools: standard::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
// }

// impl ComponentManager {
//     pub fn new() -> Self {
//         Self {
//             component_pools: standard::HashMap::default(),
//         }
//     }

//     fn prepare_pool_for<T: 'static + Component>(&mut self) {
//         let typeid = std::any::TypeId::of::<T>();
//         if !self.component_pools.contains_key(&typeid) {
//             self.component_pools.insert(typeid, Box::new(slots::SparseStorage::<T>::new()));
//         }
//     }

//     fn has_component<T: 'static + Component>(&self, entity: &Entity) -> bool {
//         match self.component_pools.get(&std::any::TypeId::of::<T>()) {
//             Some(entry) => {
//                 entry.downcast_ref::<slots::SparseStorage::<T>>()
//                     .expect("Pool does not belong to this component")
//                     .has(entity.get_slot())
//             },
//             None => {
//                 false
//             }
//         }
//     }

//     fn get_component<T: 'static + Component>(&self, entity: &Entity) -> Option<&T> {
//         if !self.has_component::<T>(entity) {
//             return None;
//         }

//         self.component_pools.get(&std::any::TypeId::of::<T>()).unwrap()
//             .downcast_ref::<slots::SparseStorage::<T>>().unwrap()
//             .get(entity.get_slot())
//     }

//     fn get_component_mut<T: 'static + Component>(&mut self, entity: &Entity) -> Option<&mut T> {
//         if !self.has_component::<T>(entity) {
//             return None;
//         }

//         self.component_pools.get_mut(&std::any::TypeId::of::<T>()).unwrap()
//             .downcast_mut::<slots::SparseStorage::<T>>().unwrap()
//             .get_mut(entity.get_slot())
//     }

//     fn get_component_unchecked<T: 'static + Component>(&self, entity: &Entity) -> &T {
//         self.component_pools.get(&std::any::TypeId::of::<T>()).unwrap()
//             .downcast_ref::<slots::SparseStorage::<T>>().unwrap()
//             .get_unchecked(entity.get_slot())
//     }

//     fn get_component_mut_unchecked<T: 'static + Component>(&mut self, entity: &Entity) -> &mut T {
//         self.component_pools.get_mut(&std::any::TypeId::of::<T>()).unwrap()
//             .downcast_mut::<slots::SparseStorage::<T>>().unwrap()
//             .get_unchecked_mut(entity.get_slot())
//     }

//     fn assign_component<T>(&mut self, entity: &Entity, comp: T)
//         where T: 'static + Component
//     {
//         self.component_pools.get_mut(&std::any::TypeId::of::<T>()).unwrap()
//             .downcast_mut::<slots::SparseStorage::<T>>().unwrap()
//             .create(entity.get_slot(), comp);
//     }
// }
