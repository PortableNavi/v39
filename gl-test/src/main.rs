#![feature(portable_simd)]
use v39::prelude::*;

#[macro_use]
extern crate log;


#[derive(Default)]
struct App
{
    gl_data: Option<GlData>,
}

impl EventReceiver for App
{
    // Init stuff
    fn reset(&mut self) -> V39Result<()> 
    {
        get_v39().timer().set_target_fps(Some(60));
        get_v39().renderer().exec_gl(|gl| unsafe {self.gl_setup(gl)})
    }

    // Quit the App on a window close event.
    fn window_close(&mut self) -> V39Result<()> 
    {
        get_v39().quit();
        Ok(())
    }

    fn window_resize(&mut self, size: (u32, u32)) -> V39Result<()> 
    {
        get_v39().renderer().exec_gl(|gl| unsafe {
            gl.viewport(0, 0, size.0 as i32, size.1 as i32);
            Ok(())
        })
    }

    // Quit the App if q is pressed.
    fn key_down(&mut self, key: input::V39Key) -> V39Result<()> 
    {
        if key == input::V39Key::Q {get_v39().quit();}
        Ok(())
    }
 
    // Each tick, render to the screen
    fn tick(&mut self, _delta: f32) -> V39Result<()> 
    {
        get_v39().renderer().exec_gl(|gl| unsafe {self.gl_draw(gl)})
    }
}


impl App
{
    unsafe fn gl_setup(&mut self, gl: &glow::Context) -> V39Result<()>
    {
        let positions = [
            0.0f32,  0.5f32, 0.0f32,
            -0.5f32,  -0.5f32, 0.0f32,
            0.5f32,  -0.5f32, 0.0f32,
        ];

        gl.viewport(0, 0, 800, 600);

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)?;
        gl.shader_source(vertex_shader, include_str!("../shaders/base.vert"));
        gl.compile_shader(vertex_shader); 

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER)?;
        gl.shader_source(fragment_shader, include_str!("../shaders/base.frag"));
        gl.compile_shader(fragment_shader);

        let shader_prog = gl.create_program()?;
        gl.attach_shader(shader_prog, vertex_shader);
        gl.attach_shader(shader_prog, fragment_shader);

        gl.link_program(shader_prog);

        gl.detach_shader(shader_prog, vertex_shader);
        gl.delete_shader(vertex_shader);
        gl.detach_shader(shader_prog, fragment_shader);
        gl.delete_shader(fragment_shader);

        println!("{:?}", to_bytes(&positions).len());

        let vbo = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, to_bytes(&positions), glow::STATIC_DRAW);

        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

        gl.bind_buffer(glow::ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        let gl_data = GlData {vao, vbo, shader: shader_prog};
        self.gl_data = Some(gl_data);
 
        Ok(())
    }

    unsafe fn gl_draw(&self, gl: &glow::Context) -> V39Result<()>
    {
        gl.clear_color(0.5, 0.5, 1.0, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        if let Some(gl_data) = self.gl_data
        {
            gl.use_program(Some(gl_data.shader));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(gl_data.vbo));
            gl.bind_vertex_array(Some(gl_data.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }

        Ok(())
    }
}


fn main() -> V39Result<()>
{
    let app = v39::init()?;
    app.event_handler().add_receiver(App::default());
    app.run()
}


#[derive(Copy, Clone)]
struct GlData
{
    shader: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
}


unsafe fn to_bytes<T>(slice: &[T]) -> &[u8]
{
    core::slice::from_raw_parts(
        slice.as_ptr() as *const u8,
        std::mem::size_of_val(slice),
    )
}

