use std::path::Path;

use graphics::{Image};


//this needs updating if graphics backend is changed!
use opengl_graphics::{Texture, TextureSettings};

pub struct TextureObject{


    image : Image,
    texture: Texture,
}


impl TextureObject {

    pub fn new_test() -> Self{

        //https://docs.piston.rs/piston_window/piston_window/struct.TextureSettings.html
        let texture_settings = TextureSettings::new();




        TextureObject{
            image: Image::new().rect([0.0, 0.0, 72.0, 51.0]),
            texture: Texture::from_path(Path::new("087glimpse.png"), &texture_settings).unwrap()
        }
    }
    
    pub fn get_draw_references(&self) -> (&Image, &Texture){
        return (&self.image, &self.texture);
    }



}


