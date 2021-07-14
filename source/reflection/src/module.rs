use crate::desc;
use super::database::*;

pub(crate) struct Instance {
    path: String,
    structs: Vec<desc::Struct>,
}

impl Instance {
    pub(crate) fn new(path: &str) -> Self {
        Instance {
            path: path.to_string(),
            structs: Vec::new(),
        }
    }

    pub(crate) fn get_path(&self) -> &String {
        return &self.path;
    } 
}

pub fn push_state(module_name: &str) -> Result<(), simple_error::SimpleError> {
        return Database::get().prepare_new_module(module_name);
}

pub fn pop_state() -> Result<(), simple_error::SimpleError> {
        return Database::get().consume_new_module();
}

pub fn test() {
        println!("{}", Database::get());
}
