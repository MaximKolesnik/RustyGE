extern crate libloading;
extern crate ctor;
extern crate reflection;
extern crate containers;
extern crate notify;

pub use ctor::ctor;
pub use ctor::dtor;
use notify::Watcher;

use std::time::Duration;
use std::{path::PathBuf, str::FromStr};
use containers::standard::*;
use std::sync::mpsc;

pub struct Loader {
    plugin_names: Vec<String>,
    search_path: PathBuf,
    libs: Vec<(PathBuf, libloading::Library)>,
    reciever: mpsc::Receiver<notify::DebouncedEvent>,
    watcher: notify::RecommendedWatcher,
}

impl Loader {
    pub fn new() -> Loader {
        let mut search_path = std::env::current_exe()
            .expect("Cannot resolve path to the executable");

        search_path.pop();

        let channel = mpsc::channel();
        let watcher = notify::watcher(channel.0, Duration::from_secs(1)).unwrap();

        Loader {
            plugin_names: Vec::new(),
            search_path,
            libs: Vec::new(),
            reciever: channel.1,
            watcher
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

            self.watcher.watch(plugin_path.as_path(), notify::RecursiveMode::NonRecursive);
        }
    }

    pub fn unload(&mut self) {
        for lib in self.libs.iter() {
            println!("Plugin {} is unloaded", lib.0.display());
        }

        self.libs.clear();
    }

    pub fn update(& mut self) {
        for event in self.reciever.try_iter() {
            match event {
                notify::DebouncedEvent::NoticeWrite(path)
                    | notify::DebouncedEvent::Chmod(path) => {
                    reflection::database::clear();
                    self.libs.retain(|val| {
                        val.0 != path
                    });

                    self.libs.push( (path.clone(), libloading::Library::new(&path).unwrap()) );
                },
                notify::DebouncedEvent::Write(path) => {
                    println!("Write to {}", path.display());
                },
                _ => {
                    println!("{:?}", event);
                }
            }
        }
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
