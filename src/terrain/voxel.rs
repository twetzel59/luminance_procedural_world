//! A module for managing the voxels in the world.

use std::{iter, slice};
use luminance::tess::{Mode, Tess, TessVertices};
use super::{Vertex, mesh_gen};
use maths::Translation;
use model::Model;
use resources::Resources;

//const VERTICES: [Vertex; 3] = [
//  ([-0.5, -0.5, 0.0], [0.0, 1.0]),
//  ([-0.5,  0.5, 0.0], [0.0, 0.0]),
//  ([ 0.5, -0.5, 0.0], [1.0, 1.0]),
//];

/// A block in the world.
#[derive(Clone, Copy, Debug)]
pub enum Block {
    Air,
    Limestone,
    Loam,
}

impl Block {
    /// Determine if the block is air.
    pub fn is_air(&self) -> bool {
        match *self {
            Block::Air => true,
            _ => false,
        }
    }
}

/// The length of one side of a cubic sector.
pub const SECTOR_SIZE: usize = 32;

// The length of an array of blocks for a sector.
const SECTOR_LEN: usize = SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE;

/// The type of sector space coordinates.
#[derive(Clone, Copy, Debug)]
pub struct SectorSpaceCoords(pub u8, pub u8, pub u8);

/// The array structure of blocks in a `Sector`.
pub struct BlockList([Block; SECTOR_LEN]);

impl BlockList {
    /// Look at the block at a specific position in sector coords.
    /// # Panics
    /// This function panics if the index is out of range.
    pub fn get(&self, pos: SectorSpaceCoords) -> &Block {
        let (x, y, z) = (pos.0 as usize, pos.1 as usize, pos.2 as usize);
        
        &self.0[x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE]
    }
}

/// An iterator over a BlockList.
pub struct BlockListIter<'a>(iter::Enumerate<slice::Iter<'a, Block>>);

type BlockListIterItem<'a> = (SectorSpaceCoords, &'a Block);

impl<'a> Iterator for BlockListIter<'a> {
    type Item = BlockListIterItem<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(i) => {
                let mut total = i.0;
                
                let z = total / (SECTOR_SIZE * SECTOR_SIZE);
                total -= z * SECTOR_SIZE * SECTOR_SIZE;
                let z = z as u8;
                
                let y = total / SECTOR_SIZE;
                total -= y * SECTOR_SIZE;
                let y = y as u8;
                
                let x = total;
                let x = x as u8;
                
                //println!("x: {}, y: {}, z: {}", x, y, z);
                
                Some((SectorSpaceCoords(x, y, z), i.1))
            },
            None => None,
        }
    }
}

impl<'a> IntoIterator for &'a BlockList {
    type Item = BlockListIterItem<'a>;
    type IntoIter = BlockListIter<'a>;
    
    fn into_iter(self) -> BlockListIter<'a> {
        BlockListIter(self.0.iter().enumerate())
    }
}

/// An individual "chunk" of the world.
pub struct Sector {
    blocks: BlockList,
    model: Model<Vertex>,
}

impl Sector {
    /// Create a sector filled with `Granite`.
    pub fn new(resources: &Resources, pos: (u32, u32, u32)) -> Sector {
        let blocks = BlockList([Block::Loam; SECTOR_LEN]);
        
        let terrain_tex = resources.terrain_tex();
        
        let vertices = mesh_gen::generate_block_vertices(&blocks, &terrain_tex.1);
        let tess = Tess::new(Mode::Triangle, TessVertices::Fill(&vertices), None);
        
        let translation = Translation::new((pos.0 as usize * SECTOR_SIZE) as f32,
                                           (pos.1 as usize * SECTOR_SIZE) as f32,
                                           (pos.2 as usize * SECTOR_SIZE) as f32);
                                           
        //println!("translation: {:?}", translation);
        
        let model = Model::with_translation(tess, terrain_tex, translation);
        
        Sector {
            blocks,
            model,
        }
    }
    
    /// Return an immutable reference to this sector's `Model`.
    pub fn model(&self) -> &Model<Vertex> {
        &self.model
    }
}
