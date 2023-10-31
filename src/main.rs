use gl::types::*;
use glfw::{Action, Context, Key};
use std::fmt;

#[derive(Debug)]
struct Vec2 {
    x: u32,
    y: u32,
}

impl Vec2 {
    fn new(x: u32, y: u32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((a, b): (i32, i32)) -> Self {
        Vec2 {
            x: a.try_into().unwrap(),
            y: b.try_into().unwrap(),
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        fmt::Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        fmt::Display::fmt(&self.y, f)?;
        f.write_str("}")
    }
}

fn main() {
    let mut screensize = Vec2::new(800, 600);
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            screensize.x,
            screensize.y,
            "OpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    window.set_framebuffer_size_callback(framebuffer_size_callback);

    let mut temp_screensize = screensize;
    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            handle_window_event(&mut window, event);
        }

        // Rendering stuf3f here:
        unsafe {
            gl::ClearColor(0.39, 0.58, 0.92, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        screensize = Vec2::from(glfw::Window::get_size(&window));

        if screensize != temp_screensize {
            println!("{:?}", screensize);
            temp_screensize = screensize;
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn framebuffer_size_callback(width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height) };
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
