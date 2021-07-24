extern crate libloading;
extern crate ctor;
extern crate reflection;
extern crate containers;

pub use ctor::ctor;
pub use ctor::dtor;

use std::process::Stdio;
use std::time::Duration;
use std::{path::PathBuf, str::FromStr};
use containers::standard::*;
use std::sync::mpsc;

pub struct Loader {
    plugin_names: Vec<String>,
    search_path: PathBuf,
    libs: Vec<(PathBuf, libloading::Library)>,
    scripts_path: PathBuf,
}

impl Loader {
    pub fn new() -> Loader {
        let mut search_path = std::env::current_exe()
            .expect("Cannot resolve path to the executable");

        search_path.pop();

        let scripts_folder = std::env::current_dir().expect("Current dir is not set");
        let scripts_folder = scripts_folder.join("scripts");
        if !scripts_folder.exists() {
            panic!("Scripts folder does not exist {}", scripts_folder.display());
        }

        Loader {
            plugin_names: Vec::new(),
            search_path,
            libs: Vec::new(),
            scripts_path: scripts_folder,
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
            let plugin_path = self.resolve_plugin_path(name);

            if !plugin_path.exists() {
                println!("Plugin {} cannot be found {}", name, plugin_path.display());
                continue;
            }

            reflection::module::push_state(plugin_path.to_string_lossy().to_mut())
                .expect("Double push of module state");

            println!("Loading {}", plugin_path.to_string_lossy().to_mut());
            match libloading::Library::new(&plugin_path) {
                Ok(lib) => {
                    self.libs.push( (plugin_path.clone(), lib) );
                    println!("Plugin {} is loaded", name);
                },
                Err(err) => println!("Cannot load plugin {}. Error {}", name, err),
            }

            reflection::module::pop_state().expect("Module state was not pushed");
        }
    }

    pub fn unload(&mut self) {
        for lib in self.libs.iter() {
            println!("Plugin {} is unloaded", lib.0.display());
        }

        self.libs.clear();
    }

    pub fn perform_hot_reload(&mut self) {
        if !self.can_be_compiled() {
            return;
        }

        reflection::database::clear();
        self.libs.clear();

        self.compile();

        for entry in self.plugin_names.iter() {
            let plugin_path = self.resolve_plugin_path(entry);
            match libloading::Library::new(&plugin_path) {
                Ok(lib) => {
                    self.libs.push( (plugin_path.clone(), lib) );
                    println!("Plugin {} is loaded", entry);
                },
                Err(err) => println!("Cannot load plugin {}. Error {}", entry, err),
            }
        }
    }

    fn can_be_compiled(&self) -> bool {
        let mut check_command = std::process::Command::new(
            self.scripts_path.join("run_cargo_at.sh"));
        check_command.arg("check").arg("source").stdout(Stdio::null());
        let out = check_command.output().expect("Cannot run check command");
        match out.status.code() {
            Some(code) => {
                if code != 0 {
                    println!("Compiled with errors!");
                    return false;
                }
            },
            None => {
                println!("Check command exited!");
                return false;
            },
        }

        return true;
    }

    fn compile(&self) {
        let mut compile_command = std::process::Command::new(
            self.scripts_path.join("cargo_build_all.sh"));
        compile_command.stdout(Stdio::null()).stderr(Stdio::null());
        let out = compile_command.output().expect("Cannot run build command");
    }

    #[cfg(target_os = "linux")]
    fn resolve_plugin_path(&self, plugin_name: &String) -> PathBuf {
        let mut plugin_path = self.search_path.clone();

        plugin_path.push(format!("lib{}.so", plugin_name));
        plugin_path
    }

    #[cfg(target_os = "windows")]
    fn resolve_plugin_path(&self, plugin_name: &String) -> PathBuf {
        let mut plugin_path = self.search_path.clone();

        plugin_path.push(format!("{}.dll", plugin_name));
        plugin_path
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
