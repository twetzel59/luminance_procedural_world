//! Utilities for managing shared data, such as images.

use std::fs::File;
use std::rc::Rc;
use luminance::pixel::RGB32F;
use luminance::texture::{Dim2, Flat, MagFilter, MinFilter, Sampler, Texture};
use png::{self, Decoder};

/// A simple resource manager that can load and provide resources.
pub struct Resources {
    terrain_tex: Rc<Texture<Flat, Dim2, RGB32F>>,
}

impl Resources {
    /// Create a new resource manager.
    /// # Panics
    /// This constructor panics if the resources
    /// could not be loaded from disk.
    pub fn new() -> Resources {
        Resources {
            terrain_tex: Rc::new(Self::load_texture(File::open("data/tex.png").unwrap())),
        }
    }
    
    /// Get terrain texture.
    pub fn terrain_tex(&self) -> Rc<Texture<Flat, Dim2, RGB32F>> {
        self.terrain_tex.clone()
    }
    
    fn load_texture(file: File) -> Texture<Flat, Dim2, RGB32F> {
        let png_decoder = Decoder::new(file);
        let (png_info, mut png_reader) = png_decoder.read_info().unwrap();
        assert_eq!(png_info.color_type, png::ColorType::RGB);
        assert_eq!(png_info.bit_depth, png::BitDepth::Eight);
        let mut png_data = vec![0; png_info.buffer_size()];
        png_reader.next_frame(&mut png_data).unwrap();
        
        //println!("size: {:?}", (png_info.width, png_info.height));
        assert_eq!(png_info.buffer_size() % 3, 0);
        let mut image = Vec::with_capacity(png_info.buffer_size() / 3);
        for i in 0..(png_info.buffer_size() / 3) {
            let x = i * 3;
            
            //println!("data: {:?}", &[png_data[x], png_data[x + 1], png_data[x + 2]]);
            image.push((png_data[x]     as f32 / 255.,
                        png_data[x + 1] as f32 / 255.,
                        png_data[x + 2] as f32 / 255.));
        }
        
        let mut sampler = Sampler::default();
        sampler.min_filter = MinFilter::Nearest;
        sampler.mag_filter = MagFilter::Nearest;
        
        let tex = Texture::<Flat, Dim2, RGB32F>::new(
                [png_info.width, png_info.height], 0, &sampler).unwrap();
        tex.upload(false, &image);
        
        tex
    }
}
