use v39::prelude::*;
use v39::math::*;


mod vol;
mod vol_mesh;
mod voxel_table;
use vol::VolCloud;


#[derive(Default)]
struct AppState(Option<(ModelId, VolCloud)>);
impl EventReceiver for AppState
{
    fn reset(&mut self) -> V39Result<()>
    {
        let cloud = VolCloud::from_file("vol/skull.vol")?;
        let r = get_v39().renderer();

        let shader = Shader::new(&[
            ShaderSource::fragment(include_str!("../shaders/default.frag")),
            ShaderSource::vertex(include_str!("../shaders/default.vert")),
        ])?;

        let shader_id = r.load_shader(shader);
        let model = vol_mesh::from_cloud(&cloud, shader_id, 50)?;

        self.0 = Some((model, cloud));
        Ok(())
    }

    fn tick(&mut self, delta: f32) -> V39Result<()> 
    {
        let r = get_v39().renderer();

        if let Some((model, _)) = &self.0
        {
            if let Some(model) = r.get_model(*model)
            {
                model.transform(|mat| {
                    glm::rotate_y(mat, 0.7*delta)
                });

                let count = r.use_model(model.id()).unwrap();

                r.exec_gl(|gl| unsafe {
                    gl.draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, 0);
                    Ok(())
                })?;
            }
        }

        Ok(())
    }

    fn window_close(&mut self) -> V39Result<()> 
    {
        get_v39().quit();
        Ok(())
    }
}


fn main() -> V39Result<()>
{
    let settings = InitProps {
        title: "Marching Cubes".into(),
        ..InitProps::default()
    };

    let v39 = v39::init(&settings)?;
    v39.event_handler().add_receiver(AppState::default());
    v39.run()
}

