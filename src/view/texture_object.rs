use std::path::Path;

use graphics::{Image,  rectangle::{square, self}};


//this needs updating if graphics backend is changed!
use opengl_graphics::{Texture, TextureSettings};

pub struct TextureObject{


    image : Image,
    texture: Texture,
}


impl TextureObject {

    pub fn new() -> Self{

        //https://docs.piston.rs/piston_window/piston_window/struct.TextureSettings.html
        let texture_settings = TextureSettings::new();




        TextureObject{
            image: Image::new().rect([0.0, 0.0, 144.0, 103.0]),
            texture: Texture::from_path(Path::new("087glimpse.png"), &texture_settings).unwrap()
        }
    }
    
    pub fn get_draw_references(&self) -> (&Image, &Texture){
        return (&self.image, &self.texture);
    }



}


