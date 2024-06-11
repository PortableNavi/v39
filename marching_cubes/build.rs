#![allow(unused)]

use math::Vec3;
use std::path::Path;
use std::fs::File;
use std::io::Write;


static VOXEL_TABLE_PATH: &str = "voxel_table.bin";


const EDGE_VERT_TABLE: [[Vec3; 3]; 8] = [
    [Vec3::new_3d(0.0, 1.0, 0.5), Vec3::new_3d(0.0, 0.5, 1.0), Vec3::new_3d(0.5, 1.0, 1.0)],
    [Vec3::new_3d(0.0, 0.5, 0.0), Vec3::new_3d(0.5, 1.0, 0.0), Vec3::new_3d(0.0, 1.0, 0.5)],
    [Vec3::new_3d(1.0, 0.5, 0.0), Vec3::new_3d(0.5, 1.0, 0.0), Vec3::new_3d(1.0, 1.0, 0.5)],
    [Vec3::new_3d(1.0, 0.5, 1.0), Vec3::new_3d(0.5, 1.0, 1.0), Vec3::new_3d(1.0, 1.0, 0.5)],
    [Vec3::new_3d(0.0, 0.5, 1.0), Vec3::new_3d(0.0, 0.0, 0.5), Vec3::new_3d(0.5, 0.0, 1.0)],
    [Vec3::new_3d(0.0, 0.0, 0.5), Vec3::new_3d(0.0, 0.5, 0.0), Vec3::new_3d(0.5, 0.0, 0.0)],
    [Vec3::new_3d(0.5, 0.0, 0.0), Vec3::new_3d(1.0, 0.5, 0.0), Vec3::new_3d(1.0, 0.0, 0.5)],
    [Vec3::new_3d(1.0, 0.0, 0.5), Vec3::new_3d(1.0, 0.5, 1.0), Vec3::new_3d(0.5, 0.0, 1.0)],
];


#[derive(Default, serde::Serialize)]
struct CubeMesh
{
    verts: Vec<Vec3>,
    indices: Vec<u32>,
    corners: Vec<Vec<usize>>,
}


fn main()
{
    if !Path::new(VOXEL_TABLE_PATH).is_file()
        || std::env::var("GENERATE_VOXEL_TABLE") == Ok("true".into())
    {
        gen_voxel_table();
    }
}


pub fn remove_from_vec(slice: &mut Vec<Vec3>, element: &Vec3) -> bool
{
    if slice.contains (element)
    {
        let index = slice.iter().position(|x| x == element).unwrap();
        slice.remove(index);
        return true;
    }

    false
}


pub fn is_active(cube: u8, corner: usize) -> bool
{
    (cube & (1 << corner)) != 0
}


pub fn active_neighbours(cube: u8, corner: usize, visited: &mut Vec<usize>)
{
    if !is_active(cube, corner) 
        || visited.contains(&corner) 
    {
        return;
    }

    visited.push(corner as usize);
    let corners = [(corner+1)%8, (corner+7)%8, (corner+4)%8];

    for c in corners
    {
        if !visited.contains(&c) && is_active(cube, c)
        {
            visited.push(c);
            active_neighbours(cube, c, visited);
        }
    }
}


fn dist(a: &Vec3, b: &Vec3) -> f64
{
    (*a - *b).lensq()
}


fn sort_verticies(slice: &mut [Vec3])
{
    for i in 1..slice.len()
    {
        let mut d = dist(&slice[i-1], &slice[i]);
        
        for j in (i+1)..slice.len()
        {
            let d2 = dist(&slice[i-1], &slice[j]);

            if d2 < d
            {
                d = d2;
                slice.swap(i, j);
            }
        }
    }
}


fn gen_voxel_table()
{
    let mut map: Vec<CubeMesh> = Vec::with_capacity(256);

    for cube in 0..=255
    {
        let mut cube_mesh = CubeMesh::default();

        let mut indices_sliced = vec![];
        let mut verts_sliced = vec![];
        let mut already_visited = vec![];

        for corner in 0..8
        {
            if !already_visited.contains(&corner)
            {
                let mut neighbours = vec![];
                active_neighbours(cube, corner, &mut neighbours);
                already_visited.extend_from_slice(&neighbours);

                if !neighbours.is_empty()
                {
                    let mut verts = vec![];
                    for n in &neighbours
                    {
                        let tri = EDGE_VERT_TABLE[*n];

                        for v in tri
                        {
                            if !remove_from_vec(&mut verts, &v)
                            {
                                verts.push(v)
                            }
                        }
                    }

                    sort_verticies(&mut verts);
                                       
                    let mut indices = vec![0, 1, 2];

                    for i in 3..verts.len()
                    {
                        let last = *indices.last().unwrap();

                        indices.push(0);
                        indices.push(last);
                        indices.push(i as u32);
                    }
                        
                    indices_sliced.push(indices);
                    verts_sliced.push(verts);
                    cube_mesh.corners.push(neighbours);
                }
            }
        }
        
        let mut offset = 0;
        for (i, mut indices) in indices_sliced.iter_mut().enumerate()
        {
            indices.iter_mut().for_each(|e| *e += offset);
            cube_mesh.indices.extend_from_slice(&indices);
            offset += verts_sliced[i].len() as u32;
        }

        cube_mesh.verts = verts_sliced.concat();
        map.push(cube_mesh);
    }

    let bytes = rmp_serde::to_vec(&map).expect("voxel table serialization failed");

    let mut output = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(VOXEL_TABLE_PATH)
        .expect(&format!("Creating {VOXEL_TABLE_PATH:?} failed"));

    output.write_all(&bytes).expect("Writing to 'voxel_table.bin' failed");
}
