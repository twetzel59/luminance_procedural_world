//! This module contains the logic for creating tesselations
//! from `Sector`.

use super::{Position, UV, Vertex};
use super::voxel::BlockList;

//const POSITIONS: [Position; 6] = {[1.0, ]};

/// Generate the mesh for a `BlockList`.
pub fn generate_block_vertices(block: &BlockList) -> Vec<Vertex> {
    let mut v = Vec::new();
    
    for i in block {
        //let x = (i.0).0 as usize;
        //let y = (i.0).1 as usize;
        //let z = (i.0).2 as usize;
        
        //println!("idx: {}, i: {:?}", x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE, i);
        
        //if !block.is_air() {
        //    generate_face(&mut v);
        //}
    }
    
    generate_face(&mut v);
    
    println!("done!");
    
    v
}

fn generate_face(v: &mut Vec<Vertex>) {
    v.push(([-0.5, -0.5, 0.0], [0.0, 1.0]));
    v.push(([-0.5,  0.5, 0.0], [0.0, 0.0]));
    v.push(([ 0.5, -0.5, 0.0], [1.0, 1.0]));
}
