//! This module contains the logic for creating tesselations
//! from `Sector`.

use super::{Position, UV, Vertex};
use super::voxel::{Block, BlockList, SECTOR_LEN};

//const POSITIONS: [Position; 6] = {[1.0, ]};

pub fn generate_block_vertices(block: &BlockList) -> Vec<Vertex> {
    let mut v = Vec::new();
    
    //v.push(([-0.5, -0.5, 0.0], [0.0, 1.0]));
    //v.push(([-0.5,  0.5, 0.0], [0.0, 0.0]));
    //v.push(([ 0.5, -0.5, 0.0], [1.0, 1.0]));
    
    for i in block {
        //println!("i: {:?}", i);
        
        //if !block.is_air() {
        //    generate_face(&mut v);
        //}
    }
    
    println!("done!");
    
    v
}

fn generate_face(v: &mut Vec<Vertex>) {
}
