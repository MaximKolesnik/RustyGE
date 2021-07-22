use std::thread;

extern crate reflection;
extern crate ecs;

fn main() {
    let mut loader = plugin_loader::Loader::new();
    loader.add_plugin("plg_test");
    loader.load();

    loop {
        loader.update();
        for obj in reflection::database::iterate_struct() {
            println!("{}", obj.get_typename());
            let var = obj.create_instance();
            var.cast_trait::<dyn ecs::System>().unwrap().update(0.0);
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
