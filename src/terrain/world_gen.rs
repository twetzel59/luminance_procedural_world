//! Procedural world generation.

use noise::{BasicMulti, MultiFractal, NoiseModule};
use super::{SECTOR_LEN, SECTOR_SIZE, SECTOR_SIZE_PAD_U32};
use super::voxel::{Block, BlockList, SectorSpaceCoords};

const SECTOR_SIZE_F: f32 = SECTOR_SIZE as f32;

/// The world generator.
#[derive(Clone)]
pub struct WorldGen {
    //perlin: Perlin,
    base_terrain: BasicMulti<f32>,
    compression: BasicMulti<f32>,
    general_height: BasicMulti<f32>,
    tree: (BasicMulti<f32>, BasicMulti<f32>),
}

impl WorldGen {
    /// Create a new `WorldGen`.
    pub fn new() -> WorldGen {
        WorldGen {
            //perlin: Perlin::new(),
            base_terrain: BasicMulti::new().set_persistence(0.1),
            compression: BasicMulti::new().set_persistence(0.05),
            general_height: BasicMulti::new().set_octaves(4).set_frequency(0.5),
            tree: (BasicMulti::new().set_frequency(0.01),
                   BasicMulti::new().set_frequency(1.0)),
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
        
        /*
        if sector.1 == 0 {
            let mut list = BlockList::new_air();
            
            for x in 0..SECTOR_SIZE_S {
                for z in 0..SECTOR_SIZE_S {
                    let (fx, fz) = (x as f32, z as f32);
                    let (s0, s2) = (sector.0 as f32, sector.2 as f32);
                    
                    let comp = (self.compression.get(
                        [(fx + SECTOR_SIZE_F * s0) * 0.005,
                         (fz + SECTOR_SIZE_F * s2) * 0.005]) + 1.0).min(1.0);
                    
                    //println!("{}", comp);
                    
                    let general_h = (self.general_height.get(
                        [(fx + SECTOR_SIZE_F * s0) * 0.0009,
                         (fz + SECTOR_SIZE_F * s2) * 0.0009]) + 1.5).min(1.0);
                    
                    let height = self.base_terrain.get(
                        [(fx + SECTOR_SIZE_F * s0) * 0.007 * comp,
                         (fz + SECTOR_SIZE_F * s2) * 0.007 * comp]) * general_h;
                    
                    let middle = SECTOR_SIZE_F / 2.;
                    
                    let highest = (middle + height * 40.).max(0.).min(SECTOR_SIZE_F) as isize;
                    
                    //println!("highest: {}", highest);
                    
                    for y in 0..highest {
                        list.set(SectorSpaceCoords::new(x, y, z),
                                 Block::Grass);
                    }
                    
                    // Trees
                    if x >= 3 && x <= SECTOR_SIZE_S - 3 && z >= 3 && z <= SECTOR_SIZE_S - 3 && highest < SECTOR_SIZE_S - 8 {
                        let tree_chance = self.tree.0.get(
                            [fx + SECTOR_SIZE_F * s0 * 1.1,
                             fz + SECTOR_SIZE_F * s2 * 1.1]);
                        
                        if tree_chance > 0.2 {
                            let tree_chance2 = self.tree.1.get(
                                [fx / 2. + SECTOR_SIZE_F * s0 * 1.1,
                                 fz / 2. + SECTOR_SIZE_F * s2 * 1.1]);
                            
                            //println!("{}", tree_chance2);
                            
                            if tree_chance2 > 0.25 {
                                //list.set(SectorSpaceCoords::new(x as u8, highest.min(SECTOR_SIZE - 1) as u8, z as u8),
                                //         Block::Loam);
                                
                                for h in 0..8 {
                                    list.set(SectorSpaceCoords::new(x, h + highest, z),
                                             Block::Tree);
                                    for dx in -2..3 {
                                        for dy in 4..8 {
                                            for dz in -2..3 {
                                                list.set(SectorSpaceCoords::new(x + dx, highest + dy, z + dz),
                                                         Block::Leaves);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            list
        } else if sector.1 == -1 {
            BlockList::new([Block::Limestone; SECTOR_LEN])
        } else {
            BlockList::new_air()
        }
        */
        
        /*
        for x in 0..SECTOR_SIZE_S {
            for z in 0..SECTOR_SIZE_S {
                for y in 0..SECTOR_SIZE_S {
                    let h = y + sector.1 as isize;
                }
            }
        }
        */
        
        let mut list = BlockList::new_air();
        
        for x in 0..SECTOR_SIZE_PAD_U32 {
            for z in 0..SECTOR_SIZE_PAD_U32 {
                for y in 0..SECTOR_SIZE_PAD_U32 {
                    let h = y as i32 + sector.1 * SECTOR_SIZE as i32;
                    
                    if h <= 0 {
                        list.set(SectorSpaceCoords::new(x, y, z),
                                 Block::Grass);
                    } else {
                        break;
                    }
                }
            }
        }
        
        list
    }
}
