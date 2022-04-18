use std::{sync::{Arc, RwLock}, time::SystemTime, cell::RefCell};

use piston::UpdateArgs;

use super::{glorper_object::{GlorperObject, Pos}, objects::test_object::TestObject};

pub struct Model{

    pub(crate) objects: Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>,
    pub(crate) glorp_pos: Pos,
    pub(crate) glorp_type: u8,
    pub(crate) glorp_radius: f32,
    pub(crate) jump_height: f32,
    start_time: RwLock<SystemTime>,
}


impl Model {
    #[inline(always)]
    pub fn update(&self, args : &UpdateArgs){
        //DO something



        const low_point : f32 = 0.1f32; //part of the playing field
        const period : f32 = 0.87f32; //in seconds
    }

    pub fn tickle_glorper(&self){
        println!("Someone pressed 'H', this tickles!");
    }

    pub fn new()-> Self{
        let ret = Model {              objects: Arc::new(RwLock::new(Vec::new())),
            glorp_pos: Pos{x: 0.0f32, y:0.0f32},
            glorp_type: 1,
            glorp_radius: 30.0f32,
            jump_height: 600.0f32
            , start_time: RwLock::new(SystemTime::now()) };

            return ret;
    }
    //TODO 
    pub fn initialize(&self){
        

        let lock = self.objects.write().unwrap();
    
        let mut lock = self.start_time.write().unwrap();
        *lock = SystemTime::now();
    }
}