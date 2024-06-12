use v39::prelude::*;
use std::sync::Mutex;


static SHADER_LOCATION: &str = "shaders";
static SHADERS_DID_COMPILE: Mutex<bool> = Mutex::new(true);


struct ShaderCheck;
impl EventReceiver for ShaderCheck
{
    fn reset(&mut self) -> V39Result<()> 
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            *SHADERS_DID_COMPILE.lock().unwrap() = !check_shader(gl);
            Ok(())
        });

        get_v39().quit();
        Ok(())
    }
}


unsafe fn check_shader(gl: &glow::Context) -> bool
{
    let mut did_compile = true;
    let shader_files = std::fs::read_dir(SHADER_LOCATION).unwrap_or_else(|_| panic!("Dir: {SHADER_LOCATION:?} was not readable"));

    for file in shader_files
    {
        let file = file.unwrap().path();
        if !file.is_file() {continue;}

        let kind = match file.extension().unwrap_or_default().to_str().unwrap_or_default()
        {
            "frag" => glow::FRAGMENT_SHADER,
            "vert" => glow::VERTEX_SHADER,
            _ => continue,
        };

        let shader = gl.create_shader(kind).unwrap();
        gl.shader_source(shader, &std::fs::read_to_string(&file).unwrap());
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader)
        {
            println!("[{file:?}]\n{}\n", gl.get_shader_info_log(shader));
            did_compile = false;
        }
    }

    did_compile
}


fn main()
{
    // Skip compile time shader check, if it was not opted into.
    if Ok("true".into()) != std::env::var("GL_SHADER_CHECK")
    {
        return;
    }

    let props = InitProps {
        title: "Compiling Shaders...".into(),
        screen_width: 200,
        screen_height: 150,
        ..InitProps::default()
    };

    let app = v39::init(&props).expect("Init Failed");
    
    app.event_handler().add_receiver(ShaderCheck);
    app.run().expect("Run Failed");

    if *SHADERS_DID_COMPILE.lock().unwrap()
    {
        panic!("Shaders did not compile");
    }
}
