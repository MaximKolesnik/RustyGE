#[macro_export]
extern crate plugin_loader;
extern crate reflection;

use plugin_loader::*;

struct Test0 {

}

struct Test1 {

}

plugin_registration!(
    println!("plugin0 registration was called"),
    reflection::registration::new_struct::<Test0>("plugin0::Test0"),
    reflection::registration::new_struct::<Test1>("plugin0::Test1")
);
