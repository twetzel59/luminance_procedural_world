//! Procedural world generation.

use noise::{NoiseModule, Perlin};
use super::SECTOR_SIZE;
use super::voxel::{Block, BlockList};

/// The world generator.
pub struct WorldGen {
    perlin: Perlin,
}

impl WorldGen {
    /// Create a new `WorldGen`.
    pub fn new() -> WorldGen {
        WorldGen {
            perlin: Perlin::new(),
        }
    }
    
    /*
    pub fn generate(&self, sector: (i32, i32, i32)) -> BlockList {
        
    }
    */
    
    pub fn generate(&self, sector: (i32, i32, i32)) -> BlockList {
        /*
        if sector.1 > 0 {
            BlockList::new(
                    [Block::Air;
                    SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])
        } else if sector.1 == 0 {
            BlockList::new(
                    [Block::Loam;
                    SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])
        } else {
            BlockList::new(
                    [Block::Limestone;
                    SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])
        }
        */
        
        if sector.1 > 0 {
            BlockList::new(
                    [Block::Air;
                    SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])
        } else if sector.1 == 0 {
            let mut array = [Block::Air; SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE];
            
            for x in 0..SECTOR_SIZE {
                for z in 0..SECTOR_SIZE {
                    let value = self.perlin.get(
                            [(x as f32 + SECTOR_SIZE as f32 * sector.0 as f32) * 0.0073,
                             (z as f32 + SECTOR_SIZE as f32 * sector.2 as f32) * 0.0073]);
                    
                    let highest = ((value.max(0.) * 40.) as usize + 2);
                    
                    for y in 0..highest {
                        if y == 1 {
                            array[x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE] = Block::Grass;
                        } else {
                            if y >= SECTOR_SIZE - 1 {
                                array[x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE] = Block::Limestone;
                                
                                break;
                            }
                            
                            array[x + y * SECTOR_SIZE + z * SECTOR_SIZE * SECTOR_SIZE] =
                            if y <= highest - 2 {
                                Block::Grass
                            } else {
                                Block::Loam
                            };
                        }
                    }
                }
            }
            
            BlockList::new(array)
        } else {
            BlockList::new(
                    [Block::Limestone;
                    SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])
        }
    }
}
