use std::{sync::{Arc, RwLock}, path::Path, collections::HashMap, f64::consts::PI};

use graphics::{Context, Rectangle, rectangle::{rectangle_by_corners}, clear, DrawState, Transformed, ellipse, ellipse_from_to, Ellipse, color::WHITE};
use imageproc::{rect::Rect, drawing::draw_filled_ellipse};
use opengl_graphics::{GlGraphics, Texture};
use piston::RenderArgs;

use crate::model::{glorper_object::GlorperObject, objects, model::Model};

use super::texture_object::TextureObject;


pub struct View{
    pub(crate) cameraPos : (f32, f32),
    pub(crate) gl: GlGraphics,
    objects: Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>,
    textureMap : HashMap<i16, TextureObject>,
    model: Arc<RwLock<Model>>,
}


        //const values are compile time values and thus don't slow down the program
        //RGBA color definition in array: red, green, blue, alpha (1- opacity)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.5];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.7];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const BLACK : [f32; 4] = [0.02, 0.02, 0.02, 1.0];
        const DARKRED: [f32; 4] = [0.3, 0.02, 0.0, 1.0];
        const DARKBLUE: [f32; 4] = [0.02, 0.01, 0.5, 1.0 ];
        const DARKERBLUE: [f32; 4] = [0.02, 0.01, 0.3, 1.0];
        const SEMIALPHAWHITE: [f32;4] = [0.94, 0.94, 0.94, 0.3];

impl View {

    pub fn new(gl: GlGraphics, objects: Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>, model:Arc<RwLock<Model>>) -> View{
        let v = View{
            cameraPos: (0.0f32, 0.0f32),
            gl: gl,
            objects : objects,
            textureMap : TextureObject::loadTextureMap(),
            model : model,
        };


        v.init();

        return v;
    }

    //might be parameterized later
    pub fn init(&self){

    }
    
    pub fn render(&mut self, args: &RenderArgs){










        


         //get relevant info here and drop the RwLockGuards before self.gl.draw if possible. Otherwise get these in the draw function




        self.gl.draw(args.viewport(), |c, gl| {
            //the functions used here, like clear/rectangle are in namespace graphics::*, the use statement makes these omittable
            clear(DARKBLUE, gl);

            View::draw_background(&c, gl, args);
            View::draw_objects(&c, gl, args, &self.objects, &self.textureMap, &self.model);



        


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
    let big_rec = Rectangle::new(DARKERBLUE);
    let rec = Rectangle::new(BLACK);






    
    let big_bkgrnd = rectangle_by_corners(10.0, 10.0, 790.0 , 590.0);
    let bkgrnd = rectangle_by_corners(18.0 , 18.0,  782.0, 582.0);
    
    //this function was called with &c, but it does not need to be dereferenced here (*c), as this is automatically done, so Object functions can be called on reference (autoderef)
    big_rec.draw(big_bkgrnd, &DrawState::default(), c.transform, gl);
    rec.draw(bkgrnd, &DrawState::default(), c.transform, gl);

}

#[inline(always)]
pub fn draw_objects( c: &Context, gl: &mut GlGraphics, args: &RenderArgs, objs: &Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>, tex_map : &HashMap<i16, TextureObject>, model : &Arc<RwLock<Model>> ){

    let read_lock = objs.read();
    if read_lock.is_err(){
        println!("Error getting read-lock for object vector!");
        return;
    }
    let vec = read_lock.unwrap();
    for object_ref in &*vec{
        let object = object_ref.read().unwrap();
        let (image, texture) = tex_map.get(&object.get_texture_id()).unwrap().get_draw_references();
        
        let transform = c.transform.trans(args.window_size[0]/2.0 - image.rectangle.unwrap()[2]/2.0, args.window_size[1]/2.0 - image.rectangle.unwrap()[3]/2.0);
        image.draw(texture, &DrawState::default(), transform, gl);
    }
    
    let model_lock = model.read().unwrap();
    //Draw Glorper
    let rect = rectangle_by_corners(800.0 * model_lock.glorp_pos.x - model_lock.glorp_radius/2.0,  600.0 - 600.0 * model_lock.glorp_pos.y - model_lock.glorp_radius/2.0,
        800.0*model_lock.glorp_pos.x + model_lock.glorp_radius/2.0, 600.0 - 600.0 * model_lock.glorp_pos.y + model_lock.glorp_radius/2.0);
    let circle =  Ellipse::new(BLUE);
    let circle_two = Ellipse::new(DARKBLUE);

    let rect_two = rectangle_by_corners(800.0 * model_lock.glorp_pos.x - model_lock.glorp_radius/1.6,  600.0 - 600.0 * model_lock.glorp_pos.y - model_lock.glorp_radius/1.6,
        800.0*model_lock.glorp_pos.x + model_lock.glorp_radius/1.6, 600.0 - 600.0 * model_lock.glorp_pos.y + model_lock.glorp_radius/1.6);
    
    circle_two.draw(rect_two, &DrawState::default(), c.transform, gl);
    circle.draw(rect, &DrawState::default(), c.transform, gl);
    


    //draw extra circle to indicate stopped
    if model_lock.stopped {
        let additional_circle = Ellipse::new(SEMIALPHAWHITE);
        let additional_rect = rectangle_by_corners(800.0 * model_lock.glorp_pos.x - model_lock.glorp_radius/(1.95 * ( model_lock.stop_duration_seconds_max / model_lock.stop_duration_seconds_current ) ),  600.0 - 600.0 * model_lock.glorp_pos.y - model_lock.glorp_radius/(1.95 * ( model_lock.stop_duration_seconds_max / model_lock.stop_duration_seconds_current ) ),
            800.0*model_lock.glorp_pos.x + model_lock.glorp_radius/(1.95 * ( model_lock.stop_duration_seconds_max / model_lock.stop_duration_seconds_current) ), 600.0 - 600.0 * model_lock.glorp_pos.y + model_lock.glorp_radius/(1.95* ( model_lock.stop_duration_seconds_max / model_lock.stop_duration_seconds_current ) ));

            additional_circle.draw(additional_rect, &DrawState::new_alpha(), c.transform, gl);
    }
    }
}

//TODO
#[inline(always)]
pub fn to_screen_coordinates(){


}