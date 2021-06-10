extern crate libloading;
extern crate ctor;

pub use ctor::ctor;
pub use ctor::dtor;

use std::{path::PathBuf, str::FromStr};

pub struct Loader {
    plugin_names: Vec<String>,
    search_path: PathBuf,
    libs: Vec<(PathBuf, libloading::Library)>
}

impl Loader {
    pub fn new() -> Loader {
        let mut search_path = std::env::current_exe()
            .expect("Cannot resolve path to the executable");

        search_path.pop();
        let bin_type = String::from_str(search_path.file_name().unwrap().to_str().unwrap()).unwrap();
        search_path.push("../../plugins");
        search_path.push(bin_type);
        if let Ok(canon_path) = search_path.canonicalize() {
            search_path = canon_path;
        }
        else {
            panic!("Cannot find plugins folder")
        }

        if !search_path.exists() {
            panic!("Cannot find plugins folder")
        }

        Loader {
            plugin_names: Vec::new(),
            search_path,
            libs: Vec::new()
        }
    }

    pub fn add_plugin(&mut self, plugin_path: &str) {
        if let Ok(name) = String::from_str(plugin_path) {
            self.plugin_names.push(name);
        }
    }

    pub fn remove_plugin(&mut self, plugin_path: &str) {
        self.plugin_names.retain(|val| *val != plugin_path);
    }

    pub fn load(&mut self) {
        for name in self.plugin_names.iter() {
            let mut plugin_path = self.search_path.clone();

            // TODO add dll support
            plugin_path.push(name);
            plugin_path.set_file_name(format!("{}{}", "lib", name));
            plugin_path.set_extension("so");

            if !plugin_path.exists() {
                println!("Plugin {} cannot be found {}", name, plugin_path.display());
            }

            match libloading::Library::new(plugin_path.clone()) {
                Ok(lib) => {
                    self.libs.push( (plugin_path, lib) );
                    println!("Plugin {} is loaded", name);
                },
                Err(err) => println!("Cannot load plugin {}. Error {}", name, err),
            }
        }
    }

    pub fn unload(&mut self) {
        for lib in self.libs.iter() {
            println!("Plugin {} is unloaded", lib.0.display());
        }

        self.libs.clear();
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        self.unload();
    }
}

#[macro_export]
macro_rules! plugin_registration{
    ($($a:stmt), *)=>{
        #[ctor]
        fn __reg_internal_() {
            $(
                $a;
            )+
        }
    }
}
