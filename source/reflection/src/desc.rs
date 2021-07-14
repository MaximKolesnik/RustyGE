use crate::{Reflected, database::Database};
use std::any::Any;
use std::ptr::{self};

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

    pub(crate) fn new<T>(name: &'static str) -> Struct
        where T: 'static
    {
        Struct {
            type_id: std::any::TypeId::of::<T>(),
            name: name,
        }
    }
}

pub struct StructBuilder<'a, T: 'static + Reflected> {
    struct_data: &'a Struct,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: 'static+ Reflected> StructBuilder<'a, T> {
    pub(crate) fn new(data: &'a Struct) -> Self {
        Self {
            struct_data: data,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn has_trait<Trait>(&mut self) -> &Self
        where Trait: 'static + Any + ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>,
            T: std::marker::Unsize<Trait>
    {
        Database::get().get_registry().register::<T, Trait>();
        self
    }
}
