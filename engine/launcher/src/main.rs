extern crate plugin_loader;

fn main() {
    let mut loader = plugin_loader::Loader::new();
    loader.add_plugin("plugin0");
    loader.add_plugin("plugin1");
    loader.load();
}
