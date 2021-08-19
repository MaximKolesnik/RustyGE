extern crate plugin_loader;

use plugin_loader::*;
use ecs::*;

plugin_registration!(
    reflection::registration::new_struct::<TestSystem>("plg_test::TestSystem")
        .has_trait::<dyn ecs::System>(),
    ecs::registration::new_component::<TestComponent>("plg_test::TestComponent")
        .has_trait::<dyn ecs::Component>()
);

#[reflect]
#[derive(Default)]
pub struct TestSystem {

}

#[reflect]
#[derive(Default)]
pub struct TestComponent {
    pub val: u64
}

#[reflect]
#[derive(Default)]
pub struct TestComponent1 {
    pub val: u64
}

impl ecs::Component for TestComponent {
    
}

impl ecs::Component for TestComponent1 {
    
}

impl ecs::System for TestSystem {
    fn prepare(&self) -> Box<dyn SystemExec> {
        SystemBuilder::new()
            .add_view(<TestComponent>::create_view())
            .add_view(<TestComponent1>::create_view())
            .finalize(|(test1_view, test2_view)| {
                test1_view.test();
                test2_view.test();
            })
    }
}
