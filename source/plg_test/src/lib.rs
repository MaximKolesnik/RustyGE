#[macro_use]
extern crate reflection_proc;
extern crate plugin_loader;
extern crate reflection;

use plugin_loader::*;

#[reflect]
struct Test0 {

}

plugin_registration!(
    println!("plg_test registration was called"),
    reflection::registration::new_struct::<Test0>("plg::Test0")
);
