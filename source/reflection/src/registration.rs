use crate::Reflected;

use super::desc::*;
use super::database::*;

pub fn new_struct<T: Reflected>(name: &'static str) -> StructBuilder<T>
    where T: 'static
{
    let s = Database::get().create_struct::<T>(name);
    StructBuilder::<T>::new(s)
}