use crate::vol::VolCloud;
use crate::voxel_table::Voxel;
use v39::prelude::*;
use v39::math::glm;
use math::Vec3;


pub fn from_cloud(vol: &VolCloud, shader_id: ShaderId, t: u8) -> V39Result<ModelId>
{
    let res = vol.resolution();
    let mut vertices = vec![];
    let mut indices = vec![];
    
    for depth in 0..*res.z()
    {
        for col in 0..*res.x()
        {
            for row in 0..*res.y()
            {
                let mut cube: u8 = 0;

                cube |= is_corner_on(vol, t, col, row, depth)      << 0;
                cube |= is_corner_on(vol, t, col, row, depth+1)    << 1;
                cube |= is_corner_on(vol, t, col+1, row, depth+1)  << 2;
                cube |= is_corner_on(vol, t, col+1, row, depth)    << 3;

                cube |= is_corner_on(vol, t, col, row+1, depth)      << 4;
                cube |= is_corner_on(vol, t, col, row+1, depth+1)    << 5;
                cube |= is_corner_on(vol, t, col+1, row+1, depth+1)  << 6;
                cube |= is_corner_on(vol, t, col+1, row+1, depth)    << 7;

                let voxel = Voxel::new(cube).to_owned();
                
                let verts = voxel.verts()
                    .iter()
                    .flat_map(|x| (*x + Vec3::new_3d((row - (row/2)) as f32, (col - (col/2)) as f32, (depth - (depth/2)) as f32))
                        .mul_scalar(0.1)
                        .buffer()
                        .to_owned())
                    .collect::<Vec<f32>>();

                vertices.extend(verts);
                indices.extend(voxel.indices().iter().map(|i| i + vertices.len() as u32)); 
            }
        }

        println!("Loading: {}%", depth as f32 / *res.z() as f32 * 100.0);
    }

    let model = Model::new(vertices, VboFormat::Position(3), indices, glow::STATIC_DRAW, shader_id, &[])?;
    Ok(get_v39().renderer().load_model(model).unwrap())
}


fn is_corner_on(vol: &VolCloud, t: u8, x: u32, y: u32, z: u32) -> u8
{
    match vol.get(x, y, z)
    {
        Some(val) if val >= t => 1,
        _ => 0,
    }
}
