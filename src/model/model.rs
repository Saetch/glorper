use std::sync::{Arc, RwLock};

use piston::UpdateArgs;

use super::{glorper_object::GlorperObject, objects::test_object::TestObject};

pub struct Model{

    pub(crate) objects: Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>

}


impl Model {
    
    pub fn update(&self, args : &UpdateArgs){
        //DO something
    }

    pub fn tickle_glorper(&self){
        println!("Someone pressed 'H', this tickles!");
    }


    //TODO 
    pub fn initialize(&self){
        

        self.objects.write().unwrap().push(Arc::new(RwLock::new(TestObject::new())));


    }
}