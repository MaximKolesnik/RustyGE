use ::desc::*;

pub struct Database {
    structs: Vec<Struct>
}

impl Database {
    pub fn create_struct<T>(&mut self, name: &'static str) -> &mut Struct
        where
            T: 'static
    {
        self.structs.push(Struct::create::<T>(name));

        for val in self.structs.iter() {
            println!("{} {:?}", val.get_typename(), val.get_type_id());
        }

        self.structs.last_mut().unwrap()
    }
}

pub static mut DATABASE: Database = Database{
    structs: Vec::new(),
};