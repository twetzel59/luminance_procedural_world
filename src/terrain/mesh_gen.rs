//! This module contains the logic for creating tesselations
//! from `Sector`.

use png::OutputInfo;
use super::{Position, UV, Vertex};
use super::voxel::{Block, BlockList, SectorSpaceCoords};

/*
const OFFSETS: [Position; 3] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
];
*/
//const BLOCK_SIZE: f32 = 

const POSITIONS: [Position; 8] = [
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 0.0, 1.0],
];

/*
const UVS: (UV, UV, UV, UV) = (
    [1.0, 1.0],
    [1.0, 0.0],
    [0.0, 0.0],
    [0.0, 1.0],
);
*/

const TILE_SIZE: f32 = 16.;

#[derive(Clone, Copy)]
enum Face {
    Back,
    Front,
    Top,
    Bottom,
    Left,
    Right,
}

/// Generate the mesh for a `BlockList`.
pub fn generate_block_vertices(block: &BlockList, texture_info: &OutputInfo) -> Vec<Vertex> {
    let mut v = Vec::new();
    
    for i in block {
        //let x = (i.0).0 as usize;
        //let y = (i.0).1 as usize;
        //let z = (i.0).2 as usize;
        
        //println!("idx: {}, i: {:?}", x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE, i);
        
        if !i.1.is_air() {
            generate_face(&mut v, i, Face::Back, texture_info);
            generate_face(&mut v, i, Face::Front, texture_info);
            generate_face(&mut v, i, Face::Top, texture_info);
            generate_face(&mut v, i, Face::Bottom, texture_info);
            generate_face(&mut v, i, Face::Left, texture_info);
            generate_face(&mut v, i, Face::Right, texture_info);
        }
    }
    
    //generate_face(&mut v);
    
    println!("done!");
    
    v
}

fn generate_face(v: &mut Vec<Vertex>, block: (SectorSpaceCoords, &Block),
                 face: Face, texture_info: &OutputInfo) {
    use self::Face::*;
    
    //Bottom => ([2, 5, 6, 1], ([1.0, 1.0], [1.0, 0.0], [0.0, 0.0], [0.0, 1.0])),
    
    let uvs = tex_coords(block.1, texture_info);
    
    let (triangles, uv) = match face {
        Back => ([0, 1, 2, 3], uvs),
        Front => ([4, 5, 6, 7], uvs),
        Top => ([5, 2, 1, 6], uvs),
        Bottom => ([3, 4, 7, 0], uvs),
        Left => ([7, 6, 1, 0], uvs),
        Right => ([3, 2, 5, 4], uvs),
    };
    
    let original = ((block.0).x() as f32, (block.0).y() as f32, (block.0).z() as f32);
    
    let mut vtx0 = (POSITIONS[triangles[0]], uv.0);
    vtx0.0[0] += original.0;
    vtx0.0[1] += original.1;
    vtx0.0[2] += original.2;
    
    let mut vtx1 = (POSITIONS[triangles[1]], uv.1);
    vtx1.0[0] += original.0;
    vtx1.0[1] += original.1;
    vtx1.0[2] += original.2;
    
    let mut vtx2 = (POSITIONS[triangles[2]], uv.2);
    vtx2.0[0] += original.0;
    vtx2.0[1] += original.1;
    vtx2.0[2] += original.2;
    
    let mut vtx3 = (POSITIONS[triangles[3]], uv.3);
    vtx3.0[0] += original.0;
    vtx3.0[1] += original.1;
    vtx3.0[2] += original.2;
    
    // Add to mesh
    v.push(vtx0);
    v.push(vtx1);
    v.push(vtx2);
    
    v.push(vtx0);
    v.push(vtx2);
    v.push(vtx3);
}

fn tex_coords(block: &Block, texture_info: &OutputInfo) -> (UV, UV, UV, UV) {
    let (width, height) = (texture_info.width as f32,
                           texture_info.height as f32);
    
    let (ru, rv) = (TILE_SIZE / width,
                    TILE_SIZE / height);
    
    let num = *block as u32 as f32 - 1.;
    
    (
        [ru * (num + 1.), rv],
        [ru * (num + 1.), 0.],
        [ru *  num,       0.],
        [ru *  num,       rv],
    )
}
