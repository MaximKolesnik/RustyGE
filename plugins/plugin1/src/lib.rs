#[macro_export]
extern crate plugin_loader;

use plugin_loader::*;

struct Test0 {

}

struct Test1 {

}

plugin_registration!(
    println!("plugin1 registration was called"),
    reflection::registration::new_struct::<Test0>("plugin1::Test0"),
    reflection::registration::new_struct::<Test1>("plugin1::Test1")
);

