use crate::Reflected;
use crate::casts::Registry;
use crate::desc;
use crate::module;
use crate::casts;

pub(crate) struct Database {
    structs: Vec<desc::Struct>,
    current_module: Option<module::Instance>,
    cast_registry: casts::Registry,
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for s in self.structs.iter() {
            write!(f, "{:?}", s.get_typename())?;
        }

        write!(f, "")
    }
}

impl Database {
    pub fn create_struct<T>(&mut self, name: &'static str) -> &mut desc::Struct
        where
            T: 'static + Reflected + Default
    {
        self.structs.push(desc::Struct::new::<T>(name));
        self.structs.last_mut().unwrap()
    }

    pub fn prepare_new_module(&mut self, name: &str) -> Result<(), simple_error::SimpleError> {
        if self.current_module.is_some() {
            bail!("Last module state was not popped. last used path {}",
                self.current_module.as_ref().unwrap().get_path());
        }

        self.current_module = Some(module::Instance::new(name));
        Ok(())
    }

    pub fn consume_new_module(&mut self) -> Result<(), simple_error::SimpleError> {
        if self.current_module.is_none() {
            bail!("No module was prepared");
        }

        self.current_module = None;

        Ok(())
    }

    pub fn get_registry(&mut self) -> &mut Registry {
        &mut self.cast_registry
    }

    pub fn get_structs(&mut self) -> &mut Vec<desc::Struct> {
        &mut self.structs
    }

    pub fn get() -> &'static mut Database {
        static mut SINGLETON: *mut Database = 0 as *mut Database;
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                let singleton = Database {
                    structs: Vec::new(),
                    current_module: None,
                    cast_registry: casts::Registry::new(),
                };

                SINGLETON = std::mem::transmute(Box::new(singleton));
            })
        }

        unsafe {
            &mut *SINGLETON
        }
    }

    fn clear(&mut self) {
        self.structs = Vec::new();
        self.cast_registry = casts::Registry::new();
    }
}

pub fn iterate_struct() -> std::slice::Iter<'static, desc::Struct> {
    Database::get().get_structs().iter()
}

// TODO remove this. used only to test hot reload
pub fn clear() {
    Database::get().clear();
}
