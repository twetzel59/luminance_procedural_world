//! This module contains the logic for creating tesselations
//! from `Sector`.

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

const UVS: (UV, UV, UV, UV) = (
    [1.0, 1.0],
    [1.0, 0.0],
    [0.0, 0.0],
    [0.0, 1.0],
);

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
pub fn generate_block_vertices(block: &BlockList) -> Vec<Vertex> {
    let mut v = Vec::new();
    
    for i in block {
        //let x = (i.0).0 as usize;
        //let y = (i.0).1 as usize;
        //let z = (i.0).2 as usize;
        
        //println!("idx: {}, i: {:?}", x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE, i);
        
        if (i.0).0 % 2 == 0 && (i.0).1 % 2 == 0 && (i.0).2 % 2 == 0 && !i.1.is_air() {
            generate_face(&mut v, i, Face::Back);
            generate_face(&mut v, i, Face::Front);
            generate_face(&mut v, i, Face::Top);
            generate_face(&mut v, i, Face::Bottom);
            generate_face(&mut v, i, Face::Left);
            generate_face(&mut v, i, Face::Right);
        }
    }
    
    //generate_face(&mut v);
    
    println!("done!");
    
    v
}

fn generate_face(v: &mut Vec<Vertex>, block: (SectorSpaceCoords, &Block), face: Face) {
    use self::Face::*;
    
    //Bottom => ([2, 5, 6, 1], ([1.0, 1.0], [1.0, 0.0], [0.0, 0.0], [0.0, 1.0])),
    
    let (triangles, uv) = match face {
        Back => ([0, 1, 2, 3], UVS),
        Front => ([4, 5, 6, 7], UVS),
        Top => ([5, 2, 1, 6], UVS),
        Bottom => ([3, 4, 7, 0], UVS),
        Left => ([7, 6, 1, 0], UVS),
        Right => ([3, 2, 5, 4], UVS),
    };
    
    let original = ((block.0).0 as f32, (block.0).1 as f32, (block.0).2 as f32);
    
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
     
    //// Old
    
    //println!("vtx0: {:?}", vtx0);
    
    /*
    use self::Face::*;
    
    // First triangle
    let offsets = match face {
        Front => ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]),
    };
    
    let xyz1 = ((block.0).0 as f32, (block.0).1 as f32, (block.0).2 as f32);
    
    let xyz2 = ([xyz1.0[0] + offsets.0[0], xyz1.0[0] + offsets.0[0], xyz1.0[0] + offsets.0[2]]
    
    
    , xyz1.1 + offsets.1, xyz1.2 + offsets.2);
    */
    
    //v.push();
    
    /*
    use self::Face::*;
    
    let coords = ((block.0).0 as f32, (block.0).1 as f32, (block.0).2 as f32);
    
    let (triangle, offsets) = match face {
        Front => ((0, 1, 2), (coords.0, coords.1, coords.2)),
    };
    
    let vertices = ((POSITIONS[triangle.0] + offsets.0, [0.0, 0.0]),
                    (POSITIONS[triangle.1] + offsets.1, [0.0, 0.0]),
                    (POSITIONS[triangle.2] + offsets.2, [0.0, 0.0]));
    
    v.push(vertices.0);
    v.push(vertices.1);
    v.push(vertices.2);
    */
    
    /*
    use self::Face::*;
    
    let coords = ((block.0).0 as f32, (block.0).1 as f32, (block.0).2 as f32);
    
    let (x1, y1, z1) = match face {
        Front => coords,
        Top => (coords.0, coords.1 + 1.0, coords.2),
    };
    
    let (x2, y2, z2) = match face {
        Front => (x1 + 1.0, y1 + 1.0, z1),
        Top => (x1 + 1.0, y1, z1 + 1.0),
    };
    
    v.push(([x1, y1, z2], [0.0, 1.0]));
    v.push(([x2, y1, z1], [1.0, 1.0]));
    v.push(([x1, y2, z1], [0.0, 0.0]));
    */
}
