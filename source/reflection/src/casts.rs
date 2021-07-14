extern crate containers;

use containers::standard::HashMap;
use std::{any::Any, any::TypeId};
use std::ptr::{self, DynMetadata};

use super::variant::*;

#[derive(Clone, Copy)]
struct VTablePtr<Trait: ?Sized + ptr::Pointee<Metadata = DynMetadata<Trait>>>(DynMetadata<Trait>);

trait TestTrait {}

struct VTableLookup<Trait: ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>> {
    tables: HashMap<TypeId, VTablePtr<Trait>>,
    _phantom: std::marker::PhantomData<Trait>,
}

impl<Trait: 'static + ?Sized + ptr::Pointee<Metadata = DynMetadata<Trait>>> VTableLookup<Trait> {
    fn new() -> Self {
        Self {
            tables: HashMap::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn register_type<Type: Any + Reflected>(&mut self)
        where Type: std::marker::Unsize<Trait>
    {
        if self.tables.contains_key(&TypeId::of::<Trait>()) {
            return;
        }
        let ptr: *const Type = ptr::null();
        let ptr = ptr as *const Trait;
        let (_, vtable) = ptr.to_raw_parts();
        let table_ptr = VTablePtr::<Trait>(vtable);
        self.tables.insert(TypeId::of::<Type>(), table_ptr);
    }

    #[allow(dead_code)]
    fn cast_owned(&self, something: Box<dyn Any>)
        -> Result<Box<Trait>, simple_error::SimpleError>
    {
        match self.tables.get(&(*something).type_id()) {
            Some(&VTablePtr(vtable)) => unsafe {
                let obj = Box::into_raw(something);
                let (data_address, _) = obj.to_raw_parts();
                let obj = std::ptr::from_raw_parts_mut(data_address, vtable);
                let obj = Box::from_raw(obj);
                Ok(obj)
            },
            None => bail!("Cannot perform the cast"),
        }
    }

    #[allow(dead_code)]
    fn cast_mut<'a>(&self, something: &'a mut dyn Any)
        -> Result<&'a mut Trait, simple_error::SimpleError>
    {
        match self.tables.get(&(*something).type_id()) {
            Some(&VTablePtr(vtable)) => {
                let obj = something as *mut dyn Any;
                let (data_address, _) = obj.to_raw_parts();
                let obj = std::ptr::from_raw_parts_mut(data_address, vtable);
                let obj = unsafe { &mut *obj };
                Ok(obj)
            },
            None => bail!("Cannot perform the cast"),
        }
    }

    #[allow(dead_code)]
    fn cast<'a>(&self, something: &'a dyn Any)
        -> Result<&'a Trait, simple_error::SimpleError>
    {
        match self.tables.get(&(*something).type_id()) {
            Some(&VTablePtr(vtable)) => {
                let obj = something as *const dyn Any;
                let (data_address, _) = obj.to_raw_parts();
                let obj = std::ptr::from_raw_parts(data_address, vtable);
                let obj = unsafe { &*obj };
                Ok(obj)
            },
            None => bail!("Cannot perform the cast"),
        }
    }

    #[allow(dead_code)]
    fn can_cast(&self, something: &dyn Any) -> bool {
        return self.tables.contains_key(&(*something).type_id())
    }
}

pub(crate) struct Registry {
    registry: HashMap<TypeId, Box<dyn Any>>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            registry: HashMap::default(),
        }
    }

    pub fn register<Type, Trait>(&mut self)
        where
        Type: 'static + Reflected + std::marker::Unsize<Trait>,
        Trait: 'static + Any + ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>
    {
        let typeid = TypeId::of::<Trait>();
        if !self.registry.contains_key(&typeid) {
            self.registry.insert(typeid, Box::new(VTableLookup::<Trait>::new()));
        }

        self.registry.get_mut(&typeid).unwrap().downcast_mut::<VTableLookup<Trait>>()
            .unwrap().register_type::<Type>();
    }

    #[allow(dead_code)]
    pub fn cast_owned<Trait>(&self, something: Box<dyn Any>)
        -> Result<Box<Trait>, simple_error::SimpleError>
        where Trait: 'static + Any + ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>
    {
        let typeid = TypeId::of::<Trait>();
        if !self.registry.contains_key(&typeid) {
            bail!("Cannot perform the cast")
        }

        let boxed: &Box<dyn Any> = self.registry.get(&typeid).unwrap();
        boxed.downcast_ref::<VTableLookup<Trait>>().unwrap().cast_owned(something)
    }

    #[allow(dead_code)]
    pub fn cast<'a, Trait>(&self, something: &'a dyn Any) 
        -> Result<&'a Trait, simple_error::SimpleError>
        where Trait: 'static + Any + ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>
    {
        let typeid = TypeId::of::<Trait>();
        if !self.registry.contains_key(&typeid) {
            bail!("Cannot perform the cast")
        }

        let boxed: &Box<dyn Any> = self.registry.get(&typeid).unwrap();
        boxed.downcast_ref::<VTableLookup<Trait>>().unwrap().cast(something)
    }

    #[allow(dead_code)]
    pub fn cast_mut<'a, Trait>(&self, something: &'a mut dyn Any) 
        -> Result<&'a mut Trait, simple_error::SimpleError>
        where Trait: 'static + Any + ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>
    {
        let typeid = TypeId::of::<Trait>();
        if !self.registry.contains_key(&typeid) {
            bail!("Cannot perform the cast")
        }

        let boxed: &Box<dyn Any> = self.registry.get(&typeid).unwrap();
        boxed.downcast_ref::<VTableLookup<Trait>>().unwrap().cast_mut(something)
    }

    #[allow(dead_code)]
    pub fn can_cast<Trait>(&self, something: & dyn Any) -> bool
        where Trait: 'static + Any + ?Sized + ptr::Pointee<Metadata = ptr::DynMetadata<Trait>>
    {
        let typeid = TypeId::of::<Trait>();
        self.registry.contains_key(&typeid) && self.registry.get(&typeid).unwrap()
            .downcast_ref::<VTableLookup<Trait>>().unwrap().can_cast(something)
    }
}

#[cfg(test)]
mod tests {
    use crate::casts::*;

    struct Test0 {}
    struct Test1 {}

    impl Reflected for Test0 {}
    impl Reflected for Test1 {}

    trait Trait0 {
        fn test_call0(&self) -> u64;
    }

    trait Trait1 {
        fn test_call1(&self) -> u64;
    }

    trait Trait2 {
        fn test_call2(&self) -> u64;
    }

    impl Trait0 for Test0 {
        fn test_call0(&self) -> u64 {
            128
        }
    }

    impl Trait2 for Test0 {
        fn test_call2(&self) -> u64 {
            512
        }
    }

    impl Trait1 for Test1 {
        fn test_call1(&self) -> u64 {
            256
        }
    }

    #[test]
    fn test0() {
        let mut registry = Registry::new();
        let test0 = Test0{};
        let test1 = Test1{};
        registry.register::<Test0, dyn Trait0>();
        registry.register::<Test0, dyn Trait2>();
        registry.register::<Test1, dyn Trait1>();

        let res0 = registry.cast::<dyn Trait0>(&test0).unwrap().test_call0();
        assert_eq!(res0, test0.test_call0(),
            "Expected result is {}, result {} ", 128, res0);

        let res1 = registry.cast::<dyn Trait2>(&test0).unwrap().test_call2();
        assert_eq!(res1, test0.test_call2(),
            "Expected result is {}, result {} ", 512, res1);

        let res2 = registry.cast::<dyn Trait1>(&test0);
        match res2 {
            Err(_) => {},
            Ok(_) => {
                assert!(false, "This cast should not be valid")
            }
        }

        let res3 = registry.cast::<dyn Trait1>(&test1).unwrap().test_call1();
        assert_eq!(res3, test1.test_call1(),
            "Expected result is {}, result {} ", 256, res3);
    }

    #[test]
    fn test1() {
        let mut registry = Registry::new();
        let mut test0 = Test0{};
        let mut test1 = Test1{};
        registry.register::<Test0, dyn Trait0>();
        registry.register::<Test0, dyn Trait2>();
        registry.register::<Test1, dyn Trait1>();

        assert!(registry.can_cast::<dyn Trait0>(&test0));
        let res0 = registry.cast_mut::<dyn Trait0>(&mut test0).unwrap().test_call0();
        assert_eq!(res0, test0.test_call0(),
            "Expected result is {}, result {} ", 128, res0);

        let res1 = registry.cast_mut::<dyn Trait2>(&mut test0).unwrap().test_call2();
        assert!(registry.can_cast::<dyn Trait2>(&test0));
        assert_eq!(res1, test0.test_call2(),
            "Expected result is {}, result {} ", 512, res1);

        let res2 = registry.cast_mut::<dyn Trait1>(&mut test0);
        match res2 {
            Err(_) => {},
            Ok(_) => {
                assert!(false, "This cast should not be valid")
            }
        }

        assert!(registry.can_cast::<dyn Trait1>(&test1));
        let res3 = registry.cast_mut::<dyn Trait1>(&mut test1).unwrap().test_call1();
        assert_eq!(res3, test1.test_call1(),
            "Expected result is {}, result {} ", 256, res3);

        assert!(!registry.can_cast::<dyn Trait2>(&test1));
    }

    #[test]
    fn test2() {
        let mut registry = Registry::new();
        let test0 = Box::new(Test0{});
        registry.register::<Test0, dyn Trait0>();

        let res0 = registry.cast_owned::<dyn Trait0>(test0).unwrap().test_call0();
        assert_eq!(res0, 128,
            "Expected result is {}, result {} ", 128, res0);
    }
}
