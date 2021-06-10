use super::desc::*;
use super::database::*;

pub fn new_struct<T>(name: &'static str) -> &'static mut Struct
where
    T: 'static
{
    unsafe {
        return DATABASE.create_struct::<T>(name);
    }
}