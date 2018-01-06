//! A module for managing the voxels in the world.

use std::{iter, slice};
use luminance::tess::{Mode, Tess, TessVertices};
use super::{Vertex, SECTOR_SIZE};
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
    Grass,
    Tree,
    Leaves,
}

impl Block {
    /// Determine if the block is air.
    pub fn is_air(&self) -> bool {
        match *self {
            Block::Air => true,
            _ => false,
        }
    }
    
    /// Determine if the block must be drawn.
    pub fn needs_rendering(&self) -> bool {
        !self.is_air()
    }
}

// The length of an array of blocks for a sector.
const SECTOR_LEN: usize = SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE;

/// The type of sector space coordinates.
#[derive(Clone, Copy, Debug)]
pub struct SectorSpaceCoords {
    x: u8,
    y: u8,
    z: u8,
}

impl SectorSpaceCoords {
    /// Create a new coordinate triple.
    /// # Panics
    /// Panics if any component is >= `SECTOR_SIZE`.
    pub fn new(x: u8, y: u8, z: u8) -> SectorSpaceCoords {
        if x as usize >= SECTOR_SIZE || y as usize >= SECTOR_SIZE || z as usize >= SECTOR_SIZE {
            panic!("SectorSpaceCoords out of range");
        }
        
        SectorSpaceCoords {
            x,
            y,
            z,
        }
    }
    
    /// If possible, create the coord for the block
    /// behind this one.
    pub fn back(&self) -> Option<SectorSpaceCoords> {
        if (self.z as usize) > 0 {
            Some(Self::new(self.x, self.y, self.z - 1))
        } else {
            None
        }
    }
    
    /// If possible, create the coord for the block
    /// in front of this one.
    pub fn front(&self) -> Option<SectorSpaceCoords> {
        if (self.z as usize) < SECTOR_SIZE - 1 {
            Some(Self::new(self.x, self.y, self.z + 1))
        } else {
            None
        }
    }
    
    /// If possible, create the coord for the block
    /// above this one.
    pub fn top(&self) -> Option<SectorSpaceCoords> {
        if (self.y as usize) < SECTOR_SIZE - 1 {
            Some(Self::new(self.x, self.y + 1, self.z))
        } else {
            None
        }
    }
    
    /// If possible, create the coord for the block
    /// below this one.
    pub fn bottom(&self) -> Option<SectorSpaceCoords> {
        if (self.y as usize) > 0 {
            Some(Self::new(self.x, self.y - 1, self.z))
        } else {
            None
        }
    }
    
    /// If possible, create the coord for the block
    /// to the left of this one.
    pub fn left(&self) -> Option<SectorSpaceCoords> {
        if (self.x as usize) > 0 {
            Some(Self::new(self.x - 1, self.y, self.z))
        } else {
            None
        }
    }
    
    /// If possible, create the coord for the block
    /// to the right of this one.
    pub fn right(&self) -> Option<SectorSpaceCoords> {
        if (self.x as usize) < SECTOR_SIZE - 1 {
            Some(Self::new(self.x + 1, self.y, self.z))
        } else {
            None
        }
    }
    
    pub fn x(&self) -> u8 { self.x }
    pub fn y(&self) -> u8 { self.y }
    pub fn z(&self) -> u8 { self.z }
}

/// The array structure of blocks in a `Sector`.
pub struct BlockList([Block; SECTOR_LEN]);

impl BlockList {
    /// Create a new `BlockList`, consuming the array
    /// of `Block`s.
    pub fn new(blocks: [Block; SECTOR_LEN]) -> BlockList {
        BlockList(blocks)
    }
    
    /// Create a new `BlockList` fulled with air.
    pub fn new_air() -> BlockList {
        BlockList([Block::Air; SECTOR_LEN])
    }

    /// Look at the block at a specific position in sector coords.
    pub fn get(&self, pos: SectorSpaceCoords) -> &Block {
        &self.0[Self::index(pos)]
    }
    
    /// Set a block at a specific position in sector coords.
    pub fn set(&mut self, pos: SectorSpaceCoords, block: Block) {
        self.0[Self::index(pos)] = block;
    }
    
    /// Determine if all blocks in the `BlockList` are air.
    pub fn needs_rendering(&self) -> bool {
        for i in self.0.iter() {
            if i.needs_rendering() {
                return true;
            }
        }
        
        false
    }
    
    // Determines the internal index of sector coords.
    fn index(pos: SectorSpaceCoords) -> usize {
        let (x, y, z) = (pos.x() as usize, pos.y() as usize, pos.z() as usize);
        
        x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE
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
                
                Some((SectorSpaceCoords::new(x, y, z), i.1))
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
    model: Option<Model<Vertex>>,
}

impl Sector {
    /// Create a sector.
    pub fn new(resources: &Resources, pos: (i32, i32, i32),
               blocks: BlockList, vertices: Vec<Vertex>) -> Sector {
        let model = if blocks.needs_rendering() {
            let terrain_tex = resources.terrain_tex();
            
            //let vertices = mesh_gen::generate_block_vertices(&blocks, &terrain_tex.1);
            let tess = Tess::new(Mode::Triangle, TessVertices::Fill(&vertices), None);
            
            let translation = Translation::new((pos.0 * SECTOR_SIZE as i32) as f32,
                                               (pos.1 * SECTOR_SIZE as i32) as f32,
                                               (pos.2 * SECTOR_SIZE as i32) as f32);
                                           
            //println!("translation: {:?}", translation);
            
            Some(Model::with_translation(tess, terrain_tex, translation))
        } else {
            None
        };

        Sector {
            blocks,
            model,
        }
    }
    
    /// Return an immutable reference to this sector's `Model`.
    /// The model may not exist, in which case `None` is returned.
    pub fn model(&self) -> Option<&Model<Vertex>> {
        self.model.as_ref()
    }
    
    /// Return this sector's `BlockList`.
    pub fn blocks(&self) -> &BlockList {
        &self.blocks
    }
}
