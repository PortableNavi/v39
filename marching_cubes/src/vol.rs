use math::Vector;
use std::path::Path;
use std::io::{Read, BufReader};


pub struct VolCloud
{
    pub resolution: Vector<3, u32>,
    pub border: u32,
    pub extent: Vector<3, f32>,
    pub data: Vec<u8>,
}


#[allow(unused)]
impl VolCloud
{
    pub fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self>
    {
        let mut file = BufReader::new(std::fs::File::open(path)?);
        let mut buff_96bit = [0u8; 4*3];
        let mut buff_32bit = [0u8; 4];
        let mut data = vec![];

        file.read_exact(&mut buff_96bit)?;

        let resolution = Vector::from_slice(&buff_96bit
            .chunks_exact(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>())
            .unwrap();

        file.read_exact(&mut buff_32bit)?;
        let border = u32::from_be_bytes(buff_32bit);

        file.read_exact(&mut buff_96bit)?;

        let extent = Vector::from_slice(&buff_96bit
            .chunks_exact(4)
            .map(|chunk| f32::from_be_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>())
            .unwrap();

        file.read_to_end(&mut data)?;

        Ok(Self{resolution, border, extent, data})
    }

    pub fn get(&self, x: u32, y: u32, z: u32) -> Option<u8>
    {
        let index = x * self.resolution.y() * self.resolution.z()
            + y * self.resolution.z()
            + z;

        self.data.get(index as usize).map(|e| *e)
    }

    pub fn getv(&self, pos: Vector<3, impl Into<u32> + Copy>) -> Option<u8>
    {
        self.get((*pos.x()).into(), (*pos.y()).into(), (*pos.z()).into())
    }

    pub fn resolution(&self) -> Vector<3, u32>
    {
        self.resolution
    }

    pub fn border(&self) -> u32
    {
        self.border
    }

    pub fn extent(&self) -> Vector<3, f32>
    {
        self.extent
    }

    pub fn data(&self) -> &[u8]
    {
        &self.data
    }
}
