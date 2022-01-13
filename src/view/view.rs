use std::{sync::Arc, path::Path};

use graphics::{Context, Rectangle, rectangle::{rectangle_by_corners}, clear, DrawState, Transformed};
use opengl_graphics::{GlGraphics};
use piston::RenderArgs;

use super::texture_object::TextureObject;


pub struct View{
    pub(crate) gl: GlGraphics,
}


        //const values are compile time values and thus don't slow down the program
        //RGBA color definition in array: red, green, blue, alpha (1- opacity)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.5];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.7];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const BLACK : [f32; 4] = [0.02, 0.02, 0.02, 1.0];
        const DARKRED: [f32; 4] = [0.3, 0.02, 0.0, 1.0];

impl View {
    
    pub fn render(&mut self, args: &RenderArgs){










        


         //get relevant info here and drop the RwLockGuards before self.gl.draw if possible. Otherwise get these in the draw function



         let r_object = TextureObject::new_test();

        self.gl.draw(args.viewport(), |c, gl| {
            //the functions used here, like clear/rectangle are in namespace graphics::*, the use statement makes these omittable
            clear(DARKRED, gl);

            View::draw_background(&c, gl, args);
            View::draw_objects(&c, gl, args, &r_object);



        


//transformations are calculatedfor the viewPort. This means, that the center of the screen will be moved to x,y, then 
//rotated, then offset an then the square is drawn with the top left corner at the given point. Then the screen is reset to the default
//position

/*
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);               
            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);*/
        });
    }


    //remove #[inline] if you want to debug inside this function!
//inline results in the code being compiled into commands and inserted wherever this function is called, instead of actually calling a function (reduces overhead, increases speed and binary size)
#[inline(always)]
pub fn draw_background(c: &Context, gl: &mut GlGraphics, args: &RenderArgs){

    let window_width = args.window_size[0];
    let window_height = args.window_size[1];
    let rec = Rectangle::new(BLACK);
    let bkgrnd = rectangle_by_corners(50.0 , 50.0,  750.0, 550.0);
    //this function was called with &c, but it does not need to be dereferenced here (*c), as this is automatically done, so Object functions can be called on reference (autoderef)
    rec.draw(bkgrnd, &DrawState::default(), c.transform, gl);
}

pub fn draw_objects( c: &Context, gl: &mut GlGraphics, args: &RenderArgs, r : &TextureObject){
    let (image, texture) = r.get_draw_references();

    let transform = c.transform.trans(args.window_size[0]/2.0 - image.rectangle.unwrap()[2]/2.0, args.window_size[1]/2.0 - image.rectangle.unwrap()[3]/2.0);

    image.draw(texture, &DrawState::default(), transform, gl);


}
}

//TODO
#[inline(always)]
pub fn to_screen_coordinates(){


}