use crate::database::Database;

use std::any::Any;
use std::ptr::{self, DynMetadata};

struct UserData
{
    data: Box<dyn std::any::Any>,
}

impl Drop for UserData {
    fn drop(&mut self) {
        println!("Dropped");
    }
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

trait RefAny {
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

impl<T: Reflected> RefAny for T {

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

    pub fn cast_trait<T: 'static>(&self) -> Result<&T, simple_error::SimpleError>
        where T: 'static + Any + ?Sized + ptr::Pointee<Metadata = DynMetadata<T>>
    {
        if self.is_user_data {
            let r = unsafe { self.data.user_data.data.as_ref() };
            return Database::get().get_registry().cast::<T>(r)
        }

        bail!("INVALID CAST");
    }

    pub fn cast_trait_mut<T: 'static>(&mut self) -> Result<&mut T, simple_error::SimpleError>
        where T: 'static + Any + ?Sized + ptr::Pointee<Metadata = DynMetadata<T>>
    {
        if self.is_user_data {
            let r = unsafe { (&mut self.data.user_data).data.as_mut() };
            return Database::get().get_registry().cast_mut::<T>(r)
        }

        bail!("INVALID CAST");
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
