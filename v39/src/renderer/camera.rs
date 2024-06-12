use crate::math::glm;


pub struct Camera
{
    view: glm::Mat4,
    fov: f32,
    near: f32,
    far: f32,
}


impl Camera
{
    pub fn new(fov: f32, near: f32, far: f32, pos: &glm::Vec3) -> Self
    {
        let view = glm::Mat4::identity().append_translation(pos);
        Self {view, fov, near, far}
    }

    pub fn view(&self) -> glm::Mat4
    {
        self.view
    }

    pub fn proj(&self, aspect: f32) -> glm::Mat4
    {
        glm::perspective(aspect, self.fov, self.near, self.far)
    }

    pub fn transform_view(&mut self, f: impl FnOnce(&glm::Mat4)->glm::Mat4)
    {
        self.view = f(&self.view);
    }
}


impl Default for Camera
{
    fn default() -> Self 
    {
        Self::new(1.57, 0.001, 100.0, &glm::Vec3::new(0.0, -0.5, -2.0))
    }
}
