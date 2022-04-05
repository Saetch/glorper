use std::sync::{Arc, RwLock};

use piston::UpdateArgs;

use super::glorper_object::GlorperObject;

pub struct Model{

    pub(crate) objects: Vec<Arc<RwLock<dyn GlorperObject>>>

}


impl Model {
    
    pub fn update(&self, args : &UpdateArgs){
        //DO something
    }

    pub fn tickle_glorper(&self){
        println!("Someone pressed 'H', this tickles!");
    }
}