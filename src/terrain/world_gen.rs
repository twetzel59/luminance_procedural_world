//! Procedural world generation.

use noise::{BasicMulti, MultiFractal, NoiseModule};
use super::SECTOR_SIZE;
use super::voxel::{Block, BlockList, SectorSpaceCoords};

/// The world generator.
pub struct WorldGen {
    //perlin: Perlin,
    noisemod: BasicMulti<f32>,
}

impl WorldGen {
    /// Create a new `WorldGen`.
    pub fn new() -> WorldGen {
        WorldGen {
            //perlin: Perlin::new(),
            noisemod: BasicMulti::new().set_persistence(0.1),
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
        
        /*
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
                    
                    let highest = (value.max(0.) * 40.) as usize + 2;
                    
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
        */
        
        if sector.1 == 0 {
            let mut list = BlockList::new_air();
            
            for x in 0..SECTOR_SIZE {
                for z in 0..SECTOR_SIZE {
                    let height = self.noisemod.get(
                        [(x as f32 + SECTOR_SIZE as f32 * sector.0 as f32) * 0.007,
                         (z as f32 + SECTOR_SIZE as f32 * sector.2 as f32) * 0.007]);
                    
                    let middle = (SECTOR_SIZE / 2) as f32;
                    
                    let highest = (middle + height * 40.).max(0.).min(SECTOR_SIZE as f32) as usize;
                    
                    //println!("highest: {}", highest);
                    
                    for y in 0..highest {
                        list.set(SectorSpaceCoords::new(x as u8, y as u8, z as u8),
                                 Block::Grass);
                    }
                }
            }
            
            list
        } else if sector.1 == -1 {
            BlockList::new([Block::Limestone; SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])
        } else {
            BlockList::new_air()
        }
    }
}
