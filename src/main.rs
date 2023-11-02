use std::ffi::CString;
use glow::*;
use glutin::{config::ConfigTemplateBuilder, display::GetGlDisplay, prelude::{GlDisplay, NotCurrentGlContext}, surface::GlSurface, context::ContextAttributesBuilder};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{window::WindowBuilder, event_loop::EventLoopBuilder, event::{Event, WindowEvent}};

fn main() {
    unsafe {
        // glutin and winit boilerplate
        let event_loop = EventLoopBuilder::new().build().unwrap();
        let window_builder = WindowBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
        let template = ConfigTemplateBuilder::new();
        let (maybe_window, gl_config) = display_builder.build(&event_loop, template, |mut configs| {
            configs.nth(0).unwrap()
        }).unwrap();
        let window = maybe_window.unwrap();
        let gl_display = gl_config.display();
        let attrs = window.build_surface_attributes(<_>::default());
        let gl_surface = gl_config.display().create_window_surface(&gl_config, &attrs).unwrap();
        let context_attributes = ContextAttributesBuilder::new().build(None);
        let gl_context = gl_display.create_context(&gl_config, &context_attributes).unwrap().make_current(&gl_surface).unwrap();

        // Create a glow context from glutin's `get_proc_address`
        let gl = Context::from_loader_function(|s| {
            let cstring = CString::new(s).unwrap();
            let cstr = cstring.as_c_str();
            gl_display.get_proc_address(cstr)
        });

        // Start rendering
        let shader_version = "#version 410";
        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        let vertex_shader_source =
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#;
        let fragment_shader_source =
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#;

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        event_loop.run(move |event, window_target| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    window_target.exit();
                }
                Event::AboutToWait => {
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    window.request_redraw();
                    gl_surface.swap_buffers(&gl_context).unwrap();
                }
                _ => (),
            }
        }).unwrap();
    }
}
