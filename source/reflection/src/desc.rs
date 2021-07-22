use crate::{Reflected, database::Database};
use crate::variant;
use std::any::Any;
use std::ptr::{self};

pub struct Struct {
    type_id: std::any::TypeId,
    name: &'static str,
    var_creator: Box<dyn Fn() -> variant::Variant>,
}

impl Struct {
    pub fn get_type_id(&self) -> &std::any::TypeId {
        return &self.type_id;
    }

    pub fn get_typename(&self) -> &str {
        return &self.name;
    }

    pub fn create_instance(&self) -> variant::Variant {
        (self.var_creator)()
    }

    pub(crate) fn new<T>(name: &'static str) -> Struct
        where T: 'static + Reflected + Default
    {
        Struct {
            type_id: std::any::TypeId::of::<T>(),
            name: name,
            var_creator: Box::new(|| -> variant::Variant {
                T::create_variant()
            }),
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
