use std::{sync::{Arc, RwLock}, time::SystemTime, cell::RefCell};

use piston::UpdateArgs;

use super::{glorper_object::{GlorperObject, Pos}, objects::test_object::TestObject};

pub struct Model{

    pub(crate) objects: Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>,
    pub(crate) glorp_pos: Pos,
    pub(crate) glorp_type: u8,
    pub(crate) glorp_radius: f64,
    start_time: SystemTime,
    diff: f64,
    highest_point:f64,
    acceleration: f64,
    speed: f64,
    bounce_speed: f64,
}

const low_point : f64 = 0.1; //part of the playing field
const period : f64 = 0.87; //in seconds

impl Model {
    //#[inline(always)]
    pub fn update(&mut self, args : &UpdateArgs){

        self.glorp_pos.add_y(self.speed*args.dt);
        if self.glorp_pos.y < low_point{
            println!("HIT!");
            let turn = low_point-self.glorp_pos.y;
            self.glorp_pos.y = low_point+turn;
            if self.bounce_speed<= 0.0{
                let relative = 1.0 + (turn / (self.speed*args.dt /2.0) );
                self.bounce_speed = -self.speed + self.acceleration*args.dt* relative;

            }
            self.speed = self.bounce_speed;
        }else{
            self.speed = self.speed - self.acceleration*args.dt;
        }
        println!("speed is : {:.3}", self.speed);


        
    }

    pub fn tickle_glorper(&self){
        println!("Someone pressed 'H', this tickles!");
    }

    pub fn new()-> Self{
        let mut ret = Model {              objects: Arc::new(RwLock::new(Vec::new())),
            glorp_pos: Pos{x: 0.5, y:0.6},
            glorp_type: 1,
            glorp_radius: 30.0,
            start_time: SystemTime::now(),
            highest_point: 0.0,                     //use later, maybe for high score?
            diff: 0.0,
            acceleration: 0.34,
            speed: 0.0,
            bounce_speed: -1.0 };
            ret.diff = ret.highest_point-low_point;
            return ret;
    }

}