pub struct Struct {
    type_id: std::any::TypeId,
    name: &'static str,
}

impl Struct {
    pub fn get_type_id(&self) -> &std::any::TypeId {
        return &self.type_id;
    }

    pub fn get_typename(&self) -> &str {
        return &self.name;
    }

    pub(crate) fn create<T>(name: &'static str) -> Struct
    where
        T: 'static
    {
        Struct {
            type_id: std::any::TypeId::of::<T>(),
            name: name,
        }
    }
}
