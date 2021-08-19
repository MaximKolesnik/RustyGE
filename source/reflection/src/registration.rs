use crate::Reflected;

use super::desc::*;
use super::database::*;

pub fn new_struct<T>(name: &str) -> StructBuilder<T>
    where T: 'static + Reflected + Default
{
    let s = Database::get().create_struct::<T>(name);
    StructBuilder::<T>::new(s)
}