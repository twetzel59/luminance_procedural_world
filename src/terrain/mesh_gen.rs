//! This module contains the logic for creating tesselations
//! from `Sector`.

use png::OutputInfo;
use super::{Position, UV, Vertex, SECTOR_SIZE};
use super::voxel::{AdjacentSectors, Block, BlockList, SectorSpaceCoords};

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
pub fn generate_block_vertices(blocks: &BlockList, adjacent: &AdjacentSectors,
                               texture_info: &OutputInfo) -> Vec<Vertex> {
    use self::Face::*;
    
    let mut v = Vec::with_capacity(SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE * 24);
    
    for i in blocks {
        if !i.1.is_air() {
            if should_create_face(Back, i.0, blocks, adjacent) {
                generate_face(&mut v, i, Back, texture_info);
            }
            
            if should_create_face(Front, i.0, blocks, adjacent) {
                generate_face(&mut v, i, Front, texture_info);
            }
            
            if should_create_face(Top, i.0, blocks, adjacent) {
                generate_face(&mut v, i, Top, texture_info);
            }
            
            if should_create_face(Bottom, i.0, blocks, adjacent) {
                generate_face(&mut v, i, Bottom, texture_info);
            }
            
            if should_create_face(Left, i.0, blocks, adjacent) {
                generate_face(&mut v, i, Left, texture_info);
            }
            
            if should_create_face(Right, i.0, blocks, adjacent) {
                generate_face(&mut v, i, Right, texture_info);
            }
        }
    }
    
    //generate_face(&mut v);
    
    //println!("done!");
    
    v
}

fn should_create_face(face: Face, coord: SectorSpaceCoords,
                      blocks: &BlockList, adjacent: &AdjacentSectors) -> bool {
    use self::Face::*;
    
    let (block_list, other_coord) = match face {
        /*
            if let Some(c) = coord.back() {
                (blocks, Some(c))
            } else {
                match adjacent.back {
                    Some(back) => (back.blocks(), Some(SectorSpaceCoords::new(coord.x(), coord.y(), 0))),
                    None => (blocks, None),
                }
            }
            */
        
        /*
        Back => {
            match coord.back() {
                Some(back) => Some(back),
                None => {
                    if let Some(sector) = adjacent.back {
                        Some(sector.blocks().get(SectorSpaceCoords::new(coord.x(), coord.y(), SECTOR_SIZE as u8 - 1)))
                    } else {
                        None
                    }
                }
            }
        },
        */
        //Back => (blocks, coord.back()),
        
        /*
        Back =>
            coord.back().map_or_else(|| {
                adjacent.back.map_or_else(|| (blocks, None), |back| {
                    (back.blocks(), Some(SectorSpaceCoords::new(coord.x(), coord.y(), 0)))
                })
            }, |c| (blocks, Some(c))),
        Front => 
            coord.front().map_or_else(|| {
                adjacent.front.map_or_else(|| (blocks, None), |front| {
                    (front.blocks(), Some(SectorSpaceCoords::new(coord.x(), coord.y(), SECTOR_SIZE as u8 - 1)))
                })
            }, |c| (blocks, Some(c))),
        Top =>
            coord.top().map_or_else(|| {
                adjacent.top.map_or_else(|| (blocks, None), |top| {
                    (top.blocks(), Some(SectorSpaceCoords::new(coord.x(), 0, coord.z())))
                })
            }, |c| (blocks, Some(c))),
        Bottom =>
            coord.bottom().map_or_else(|| {
                adjacent.bottom.map_or_else(|| (blocks, None), |bottom| {
                    (bottom.blocks(), Some(SectorSpaceCoords::new(coord.x(), SECTOR_SIZE as u8 - 1, coord.z())))
                })
            }, |c| (blocks, Some(c))),
        Left =>
            coord.left().map_or_else(|| {
                adjacent.left.map_or_else(|| (blocks, None), |left| {
                    (left.blocks(), Some(SectorSpaceCoords::new(SECTOR_SIZE as u8 - 1, coord.y(), coord.z())))
                })
            }, |c| (blocks, Some(c))),
        Right =>
            coord.right().map_or_else(|| {
                adjacent.right.map_or_else(|| (blocks, None), |right| {
                    (right.blocks(), Some(SectorSpaceCoords::new(0, coord.y(), coord.z())))
                })
            }, |c| (blocks, Some(c))),
        */
        
        /*
        Back => (blocks, coord.back()),
        Front => (blocks, coord.front()),
        Top => (blocks, coord.top()),
        Bottom => (blocks, coord.bottom()),
        Left => (blocks, coord.left()),
        Right => (blocks, coord.right()),
        */
        
        Back =>
            coord.back().map_or_else(|| {
                (adjacent.back.blocks(), Some(SectorSpaceCoords::new(coord.x(), coord.y(), SECTOR_SIZE as u8 - 1)))
            }, |c| (blocks, Some(c))),
        Front => 
            coord.front().map_or_else(|| {
                (adjacent.front.blocks(), Some(SectorSpaceCoords::new(coord.x(), coord.y(), 0)))
            }, |c| (blocks, Some(c))),
        Top =>
            coord.top().map_or_else(|| {
                (adjacent.top.blocks(), Some(SectorSpaceCoords::new(coord.x(), 0, coord.z())))
            }, |c| (blocks, Some(c))),
        Bottom =>
            coord.bottom().map_or_else(|| {
                (adjacent.bottom.blocks(), Some(SectorSpaceCoords::new(coord.x(), SECTOR_SIZE as u8 - 1, coord.z())))
            }, |c| (blocks, Some(c))),
        Left =>
            coord.left().map_or_else(|| {
                (adjacent.left.blocks(), Some(SectorSpaceCoords::new(SECTOR_SIZE as u8 - 1, coord.y(), coord.z())))
            }, |c| (blocks, Some(c))),
        Right =>
            coord.right().map_or_else(|| {
                (adjacent.right.blocks(), Some(SectorSpaceCoords::new(0, coord.y(), coord.z())))
            }, |c| (blocks, Some(c))),
    };
    
    other_coord.map_or(true, |c| !block_list.get(c).needs_rendering())
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
