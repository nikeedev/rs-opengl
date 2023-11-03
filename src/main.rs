use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::fmt;
use std::mem;
use std::ptr;
use std::str;
use std::os::raw::c_void;

mod shader;

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

static VS_SRC: &'static str = "
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
out vec4 color;

void main() {
    gl_Position = vec4(aPos, 1.0);
    color = aColor;
}";

static FS_SRC: &'static str = "
#version 330 core
in vec4 color;
out vec4 FragColor;

void main() {
    FragColor = color;
}";

static VERTEX_DATA: [GLfloat; 18] = [
    // positions         // colors
    0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
   -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
    0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
];

fn main() {
    let mut screensize = Vec2::new(800, 600);
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(
            screensize.x,
            screensize.y,
            "OpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // the supplied function must be of the type:
    // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
    gl::load_with(|s| window.get_proc_address(s));

    // loading a specific function pointer
    gl::Viewport::load_with(|s| window.get_proc_address(s));

    // Create a windowed mode window and its OpenGL context
    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Create GLSL shaders
    let our_shader = shader::Shader::new("shaders/shader.vs", "shaders/shader.fs");

    // Triangle - Vertex
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        our_shader.useProgram();
        // Specify the layout of the vertex data

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        // postition attrbute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            stride,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);


        gl::BindVertexArray(0);
    };

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

            // Rendering stuff here:

            our_shader.useProgram();
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);
        };

        screensize = Vec2::from(glfw::Window::get_size(&window));

        if screensize != temp_screensize {
            println!("{:?}", screensize);
            temp_screensize = screensize;
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteProgram(our_shader.ID);
    };

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

