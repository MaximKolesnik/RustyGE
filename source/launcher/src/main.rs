#[macro_use]
extern crate reflection_proc;
extern crate plugin_loader;

use plugin_loader::*;
use reflection::Reflected;

#[reflect]
struct Test0 {
    data: u8,
}

struct Test1 {
    data: u8,
}

impl Default for Test0 {
    fn default() -> Test0 {
        Test0 {
            data: 0
        }
    }
}

impl Copy for Test0 {}

impl Clone for Test0 {
    fn clone(&self) -> Test0 {
        *self
    }
}

trait CoolTrait {
    fn cool(&self);
}

impl CoolTrait for Test0 {
    fn cool(&self) {
        println!("COOL TRAIT");
    }
}

plugin_registration!(
    println!("main registration was called"),
    reflection::registration::new_struct::<Test0>("main::Test0")
        .has_trait::<dyn CoolTrait>()
);

fn main() {
    let mut loader = plugin_loader::Loader::new();
    loader.add_plugin("plg_test");
    loader.load();
    

    let var = Test0::create_variant();
    var.cast_trait::<dyn CoolTrait>().unwrap().cool();
}
