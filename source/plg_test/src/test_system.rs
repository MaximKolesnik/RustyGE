extern crate plugin_loader;

use plugin_loader::*;

plugin_registration!(
    reflection::registration::new_struct::<TestSystem>("plg_test::TestSystem")
        .has_trait::<dyn ecs::System>()
);

#[reflect]
#[derive(Default)]
pub struct TestSystem {

}

impl ecs::System for TestSystem {
    fn update(&self, dt: f32) {
        println!("TEstPlugin system update call!!!!!!!!!!!!!!!!")
    }
}
