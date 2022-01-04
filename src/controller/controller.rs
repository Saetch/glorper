use std::sync::Arc;

use piston::{ButtonArgs, ButtonState, Button, Key};

use crate::model::model::Model;

pub struct Controller{
    pub(crate) model : Arc<Model>
}



impl Controller {
    
    pub fn compute_input(&self, args: &ButtonArgs){
        if args.state == ButtonState::Release{                          //only compute key presses, not 
            return;
        }

        match args.button{
            //add the type of input to read here
            Button::Keyboard(key) => self.compute_keyboard(key),
            _ => (),
        }
    }



    fn compute_keyboard(& self, key: Key){

        match key{
            Key::H => self.model.tickle_glorper(),
            _ =>(),
        }

    }
}

