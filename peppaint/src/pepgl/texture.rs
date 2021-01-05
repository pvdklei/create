use crate::{ Uint, Color, Float, Int };
use image::{ RgbaImage, DynamicImage };
use gl;
use std::os::raw::c_void;

const DEFAULT_TEX_PARAMS: TextureParams = TextureParams {
    filtering: TexFiltering::Linear,
    wrapping: TexWrapping::Repeat
};

pub struct Texture {
    id: Uint,
}

impl Texture {

    pub fn from_path(path: &str) -> Self {
        let img = image::open(path).expect("Could not open image...");
        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            img => img.to_rgba()
        };
        Self::rbga_with_params(img, DEFAULT_TEX_PARAMS)
    }

    // WHITE TEXTURE HERE

    pub fn rbga_with_params(img: RgbaImage, params: TextureParams) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
    
            let (width, height) = img.dimensions();
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA as Int, 
                width as Int, 
                height as Int, 
                0, 
                gl::RGBA, 
                gl::UNSIGNED_BYTE, 
                img.into_raw().as_ptr() as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
    
            params.set();
    
            Self { id }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

pub struct TextureParams {
    wrapping: TexWrapping,
    filtering: TexFiltering
}

impl TextureParams {
    fn set(&self) {
        use TexWrapping::*;
        use TexFiltering::*;

        unsafe {
            let wrap = match self.wrapping {
                Repeat => gl::REPEAT,
                MirrorRepeat => gl::MIRRORED_REPEAT,
                ClampEdge => gl::CLAMP_TO_EDGE,
                Constant(c) => {
                    let c = &[c.0, c.1, c.2, c.3] as *const Float;
                    gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, c); 
                    gl::CLAMP_TO_BORDER
                }
            } as Int;
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap);
    
            match self.filtering {
                Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as Int);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as Int);
                },
                Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as Int);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as Int);
                }
            };
        }
    }
}

pub enum TexWrapping {
    Repeat,
    MirrorRepeat,
    ClampEdge,
    Constant(Color)
}

pub enum TexFiltering {
    Linear,
    Nearest
}