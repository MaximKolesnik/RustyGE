fn main() {
    let mut loader = plugin_loader::Loader::new();
    loader.add_plugin("plg_test");
    loader.load();
}
