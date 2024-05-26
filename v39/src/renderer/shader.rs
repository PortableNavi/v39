use crate::prelude::*;
use crate::math::*;


#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShaderKind
{
    VertexShader = glow::VERTEX_SHADER,
    FragmentShader = glow::FRAGMENT_SHADER,
}


#[derive(Clone, PartialEq, Debug)]
pub enum UniformValue
{
    F32(f32),
    U32(u32),
    I32(i32),

    F32Vec2(f32, f32),
    U32Vec2(u32, u32),
    I32Vec2(i32, i32),

    F32Vec3(f32, f32, f32),
    U32Vec3(u32, u32, u32),
    I32Vec3(i32, i32, i32),

    F32Vec4(f32, f32, f32, f32),
    U32Vec4(u32, u32, u32, u32),
    I32Vec4(i32, i32, i32, i32),
    
    Mat4(Mat4),
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

    pub fn set_uniform(&self, name: &str, val: UniformValue) -> bool
    {
        let result = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.use_program(Some(self.program));

            let loc = match gl.get_uniform_location(self.program, name)
            {
                Some(loc) => loc,
                None => return Err(V39Error::Renderer(format!("No such uniform: {name:?}"))),
            };
            
            match val
            {
                UniformValue::F32(x) => gl.uniform_1_f32(Some(&loc), x),
                UniformValue::U32(x) => gl.uniform_1_u32(Some(&loc), x),
                UniformValue::I32(x) => gl.uniform_1_i32(Some(&loc), x),
                UniformValue::F32Vec2(x, y) => gl.uniform_2_f32(Some(&loc), x, y),
                UniformValue::U32Vec2(x, y) => gl.uniform_2_u32(Some(&loc), x, y),
                UniformValue::I32Vec2(x, y) => gl.uniform_2_i32(Some(&loc), x, y),
                UniformValue::F32Vec3(x, y, z) => gl.uniform_3_f32(Some(&loc), x, y, z),
                UniformValue::U32Vec3(x, y, z) => gl.uniform_3_u32(Some(&loc), x, y, z),
                UniformValue::I32Vec3(x, y, z) => gl.uniform_3_i32(Some(&loc), x, y, z),
                UniformValue::F32Vec4(x, y, z, d) => gl.uniform_4_f32(Some(&loc), x, y, z, d),
                UniformValue::U32Vec4(x, y, z, d) => gl.uniform_4_u32(Some(&loc), x, y, z, d),
                UniformValue::I32Vec4(x, y, z, d) => gl.uniform_4_i32(Some(&loc), x, y, z, d),
                UniformValue::Mat4(mat) => gl.uniform_matrix_4_f32_slice(Some(&loc), false, mat.as_slice()),
            }

            gl.use_program(None);
            Ok(())
        });

        match result
        {
            Ok(_) => true,
            Err(e) => {
                error!("{e}");
                false
            }
        }
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

