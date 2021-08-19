use crate::ComponentStorage;

pub fn new_component<T>(name: &str) -> reflection::desc::StructBuilder<T>
    where T: 'static + reflection::Reflected + crate::Component + Default
{
    let storage_name = format!("ComponentStorage<{}>", name);
    reflection::registration::new_struct::<ComponentStorage<T>>(
        storage_name.as_str())
        .has_trait::<dyn crate::ComponentStorageAccessor>();

    reflection::registration::new_struct::<T>(name)
        .has_trait::<dyn crate::ComponentPrivate>()
}