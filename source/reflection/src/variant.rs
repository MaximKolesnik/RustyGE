use crate::database::Database;

use std::any::Any;
use std::ptr::{self, DynMetadata};

struct UserData
{
    data: Box<dyn std::any::Any>,
}

#[repr(C)]
union VarData {
     u8_data: u8,
     u16_data: u16,
     u32_data: u32,
     u64_data: u64,
     u128_data: u128,
     user_data: std::mem::ManuallyDrop<UserData>,
}

pub struct Variant {
    data: VarData,
    // TODO fix this
    is_user_data: bool
}

pub trait Reflected {
    fn create_variant() -> Variant
        where Self: 'static + Default
    {
        Variant::from_value_impl_user_data_static::<Self>()
    }

    fn copy_to_variant(&self) -> Variant
        where Self: 'static + Clone
    {
        Variant::from_value_impl_user_data_copy(self)
    }
}

impl Reflected for u8 {
    fn create_variant() -> Variant {
        Variant::from_value_impl_built_in::<u8>(&0)
    }

    fn copy_to_variant(&self) -> Variant {
        Variant::from_value_impl_built_in::<u8>(self)
    }
}

impl Reflected for u16 {
    fn create_variant() -> Variant {
        Variant::from_value_impl_built_in::<u16>(&0)
    }

    fn copy_to_variant(&self) -> Variant {
        Variant::from_value_impl_built_in::<u16>(self)
    }
}

impl Reflected for u32 {
    fn create_variant() -> Variant {
        Variant::from_value_impl_built_in::<u32>(&0)
    }

    fn copy_to_variant(&self) -> Variant {
        Variant::from_value_impl_built_in::<u32>(self)
    }
}

impl Reflected for u64 {
    fn create_variant() -> Variant {
        Variant::from_value_impl_built_in::<u64>(&0)
    }

    fn copy_to_variant(&self) -> Variant {
        Variant::from_value_impl_built_in::<u64>(self)
    }
}

impl Reflected for u128 {
    fn create_variant() -> Variant {
        Variant::from_value_impl_built_in::<u128>(&0)
    }

    fn copy_to_variant(&self) -> Variant {
        Variant::from_value_impl_built_in::<u128>(self)
    }
}

impl Variant {
    fn from_value_impl_built_in<T: Reflected + Copy>(val: &T) -> Variant {
        println!("from_value_impl_built_in");

        unsafe {
            let mut v = Variant {
                data: VarData{ u8_data: 0 },
                is_user_data: false,
            };

            let casted = std::mem::transmute::<*mut u8, *mut T>(&mut v.data.u8_data);

            *casted = *val;
            v
        }
    }

    fn from_value_impl_user_data_copy<T>(val: &T) -> Variant
        where T: 'static + Reflected + Clone
    {
        println!("from_value_impl_user_data");

        let v = Variant {
            data: VarData{
                user_data: std::mem::ManuallyDrop::new(UserData{ data: Box::new(val.clone()) })
                },
            is_user_data: true,
        };

        v
    }

    fn from_value_impl_user_data_static<T: 'static + Reflected + ?Sized + Default>() -> Variant {
        println!("from_value_impl_user_data");

        let v = Variant {
            data: VarData{
                user_data: std::mem::ManuallyDrop::new(UserData{ data: Box::new(T::default()) })
                },
            is_user_data: true,
        };

        v
    }

    pub fn can_cast_trait<T: 'static>(&self) -> bool
        where T: 'static + Any + ?Sized + ptr::Pointee<Metadata = DynMetadata<T>>
    {
        if self.is_user_data {
            let r = unsafe { self.data.user_data.data.as_ref() };
            return Database::get().get_registry().can_cast::<T>(r);
        }

        return false;
    }

    pub fn cast_trait<T>(&self) -> Result<&T, simple_error::SimpleError>
        where T: 'static + Any + ?Sized + ptr::Pointee<Metadata = DynMetadata<T>>
    {
        if self.is_user_data {
            let r = unsafe { self.data.user_data.data.as_ref() };
            return Database::get().get_registry().cast::<T>(r)
        }

        bail!("INVALID CAST");
    }

    pub fn cast_trait_mut<T>(&mut self) -> Result<&mut T, simple_error::SimpleError>
        where T: 'static + Any + ?Sized + ptr::Pointee<Metadata = DynMetadata<T>>
    {
        if self.is_user_data {
            let r = unsafe { (&mut self.data.user_data).data.as_mut() };
            return Database::get().get_registry().cast_mut::<T>(r)
        }

        bail!("INVALID CAST");
    }

    pub fn into_boxed_trait<T>(mut self) -> Result<Box<T>, simple_error::SimpleError>
        where T: 'static + Any + ?Sized + ptr::Pointee<Metadata = DynMetadata<T>>
    {
        let uninit = std::mem::MaybeUninit::uninit();
        let man_drop = unsafe {std::mem::replace(&mut self.data.user_data, uninit.assume_init()) };
        
        std::mem::forget(self);

        let data = std::mem::ManuallyDrop::into_inner(man_drop);

        return Database::get().get_registry().cast_owned::<T>(data.data);
    }

    pub fn into_raw_object<T: 'static>(mut self) -> Option<T>
    {
        let uninit = std::mem::MaybeUninit::uninit();
        let man_drop = unsafe {std::mem::replace(&mut self.data.user_data, uninit.assume_init()) };
        
        std::mem::forget(self);

        let data = std::mem::ManuallyDrop::into_inner(man_drop);

        match data.data.downcast::<T>() {
            Ok(casted) => {
                Some(Box::into_inner(casted))
            },
            Err(_) => {
                None
            }
        }
    }

    pub fn get_type_id(&self) -> Option<std::any::TypeId> {
        if self.is_user_data {
            let typeid = unsafe {
                (*self.data.user_data.data).type_id()
            };
            return Some(typeid);
        }

        None
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        if self.is_user_data {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.data.user_data);
            }
        }
    }
}
