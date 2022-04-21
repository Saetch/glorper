use std::sync::{Arc, RwLock};

use piston::{ButtonArgs, ButtonState, Button, Key};


//  crate::folder::file::Struct
use crate::model::model::Model;

pub struct Controller{
    pub(crate) model : Arc<RwLock<Model>>,
}



impl Controller {
    
    pub fn compute_input(&self, args: &ButtonArgs){
        if args.state == ButtonState::Release{                          //only compute key presses, not 
            match args.button{
                Button::Keyboard(key) => self.compute_key_up(key),
                _ => (),
            }
        }
        else{
            match args.button{
            
                //add the type of input to read here
                Button::Keyboard(key) => self.compute_keyboard(key),
                _ => (),
            }
        }

    }

    fn compute_key_up(&self, key: Key){
        match key{
            Key::Space => self.model.write().unwrap().continue_ball(),
            _ => (),
        }
    }

    fn compute_keyboard(& self, key: Key){

        match key{
            Key::H => self.model.read().unwrap().tickle_glorper(),
            Key::Space => self.model.write().unwrap().stop_ball(),
            _ =>(),
        }

    }
}

