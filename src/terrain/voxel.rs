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
    Granite,
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

/// The length of an array of blocks for a sector.
pub const SECTOR_LEN: usize = SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE;

/// The type of chunk space coordinates.
#[derive(Debug)]
pub struct ChunkSpaceCoords(pub u8, pub u8, pub u8);

/// The array structure of blocks in a `Sector`.
pub struct BlockList([Block; SECTOR_LEN]);

/// An iterator over a BlockList.
pub struct BlockListIter<'a>(iter::Enumerate<slice::Iter<'a, Block>>);

type BlockListIterItem<'a> = (ChunkSpaceCoords, &'a Block);

impl<'a> Iterator for BlockListIter<'a> {
    type Item = BlockListIterItem<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(i) => {
                //let x = 
                
                Some((ChunkSpaceCoords(0, 0, 0), i.1))
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
    pub fn new(resources: &Resources) -> Sector {
        let blocks = BlockList([Block::Granite; SECTOR_LEN]);
        
        let vertices = mesh_gen::generate_block_vertices(&blocks);
        let tess = Tess::new(Mode::Triangle, TessVertices::Fill(&vertices), None);
        
        let model = Model::with_translation(tess,
                                            resources.terrain_tex(),
                                            Translation::new(1., 0., -1.));
        
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
