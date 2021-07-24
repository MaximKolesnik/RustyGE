use std::thread;

extern crate reflection;
extern crate ecs;
extern crate glfw;
use glfw::{Action, Context, Key};

fn set_working_dir() {
    let binary_path = std::env::current_exe().expect("Cannot resolve binary path");
    let wd = binary_path.parent().unwrap().parent().unwrap().parent().unwrap();
    std::env::set_current_dir(wd).expect("Cannot set working directory");
}

fn main() {
    set_working_dir();
    println!("{}", std::env::current_dir().unwrap().display());
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    let mut loader = plugin_loader::Loader::new();
    loader.add_plugin("plg_test");
    loader.load();

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                glfw::WindowEvent::Key(Key::F12, _, Action::Press, _) => {
                    loader.perform_hot_reload();
                },
                _ => {},
            }
        }

        for obj in reflection::database::iterate_struct() {
            println!("{}", obj.get_typename());
            let var = obj.create_instance();
            var.cast_trait::<dyn ecs::System>().unwrap().update(0.0);
        }
        thread::sleep(std::time::Duration::from_millis(500));
    }
}
