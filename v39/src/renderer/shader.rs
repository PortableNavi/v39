use crate::prelude::*;


#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShaderKind
{
    VertexShader = glow::VERTEX_SHADER,
    FragmentShader = glow::FRAGMENT_SHADER,
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ShaderSource<'a>
{
    pub source: &'a str,
    pub kind: ShaderKind,
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Shader
{
    program: glow::Program,
}


impl<'a> ShaderSource<'a>
{
    pub fn new(source: &'a str, kind: ShaderKind) -> Self
    {
        Self {source, kind}
    }

    pub fn vertex(source: &'a str) -> Self
    {
        Self::new(source, ShaderKind::VertexShader)
    }

    pub fn fragment(source: &'a str) -> Self
    {
        Self::new(source, ShaderKind::FragmentShader)
    }
}


impl Shader
{
    pub fn new(sources: &[ShaderSource]) -> V39Result<Self>
    {
        let mut program = None;

        get_v39().renderer().exec_gl(|gl| unsafe {
            program = Some(gl.create_program()?);
            let program = program.unwrap();
            
            let mut shaders = vec![];

            for src in sources
            {
                let shader = gl.create_shader(src.kind as u32)?;
                gl.shader_source(shader, src.source);
                gl.compile_shader(shader);

                if !gl.get_shader_compile_status(shader)
                {
                    return Err(V39Error::Renderer("Shader Failed to compile".into()));
                }

                gl.attach_shader(program, shader);
                shaders.push(shader);
            }
            
            gl.link_program(program);

            for shader in shaders
            {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            Ok(())
        })?;

        match program
        {
            Some(program) => Ok(Self {program}),
            None => Err(V39Error::Renderer("Shader program creation failed".into())),   //UNREACHABLE
        }
    }

    pub(crate) fn program(&self) -> glow::Program
    {
        self.program
    }
}


impl Drop for Shader
{
    fn drop(&mut self) 
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.delete_program(self.program);      
            Ok(())
        });
    }
}

