use math::Vec3;
use std::sync::{OnceLock, Once};


#[allow(unused)]
#[derive(serde::Deserialize, Debug)]
pub struct Voxel
{
    verts: Vec<Vec3>,
    indices: Vec<u32>,
    corners: Vec<Vec<usize>>,
}


impl Voxel
{
    pub fn new(index: u8) -> &'static Self
    {
        static TABLE: OnceLock<Vec<Voxel>> = OnceLock::new();
        static TABLE_INIT: Once = Once::new();

        TABLE_INIT.call_once(||{
            let table = rmp_serde::from_slice(include_bytes!("../voxel_table.bin")).expect("voxel table deserialization failed");
            let _ = TABLE.set(table);
        });

        &TABLE.get().unwrap()[index as usize]
    }

    pub fn vert_array(&self) -> Vec<f32>
    {
        self.verts.iter()
            .flat_map(|v| v.buffer())
            .map(|e| *e)
            .collect()
    }

    pub fn verts(&self) -> Vec<Vec3>
    {
        self.verts.clone()
    }

    pub fn indices(&self) -> Vec<u32>
    {
        self.indices.clone()
    }

    pub fn corners(&self) -> Vec<Vec<usize>>
    {
        self.corners.clone()
    }
}
