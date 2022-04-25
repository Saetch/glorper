use std::{sync::{Arc, RwLock}, time::{SystemTime, Duration}, cell::RefCell};

use piston::UpdateArgs;

use super::{glorper_object::{GlorperObject, Pos}, objects::test_object::TestObject, field_thing::FieldThing};

pub struct Model{

    pub(crate) objects: Arc<RwLock<Vec<Arc<RwLock<dyn GlorperObject>>>>>,
    pub(crate) glorper_field_things: Arc<RwLock<Vec<Arc<RwLock<dyn FieldThing>>>>>,
    pub(crate) glorp_pos: Pos,
    pub(crate) glorp_type: u8,
    pub(crate) glorp_radius: f64,
    start_time: SystemTime,
    highest_point:f64,
    acceleration: f64,
    speed: f64,
    bounce_speed: f64,
    ingame_height: f64,
    pub(crate) stop_duration_seconds_max: f64,
    pub(crate) stop_duration_seconds_current: f64,
    pub(crate) stopped: bool,
}

const low_point : f64 = 0.1; //part of the playing field
const period : f64 = 0.87; //in seconds

impl Model {
    //#[inline(always)]
    pub fn update(&mut self, args : &UpdateArgs){
        if self.stopped{
            self.stop_duration_seconds_current+= args.dt;
            if self.stop_duration_seconds_current > self.stop_duration_seconds_max {
                self.continue_ball();
            }
        }else{
            //calculate new position and speed
            self.glorp_pos.add_y(self.speed*args.dt);
            if self.glorp_pos.y < low_point{
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
        }
    

     
        
    }

    pub fn tickle_glorper(&self){
        println!("Someone pressed 'H', this tickles!");
    }

    pub fn stop_ball(&mut self){
        self.stopped = true;
    }

    pub fn continue_ball(&mut self){
        self.stopped = false;
        self.stop_duration_seconds_current = 0.0;
    }

    pub fn new()-> Self{
        let mut ret = Model {              objects: Arc::new(RwLock::new(Vec::new())),
            glorper_field_things: Arc::new(RwLock::new(Vec::new())),
            glorp_pos: Pos{x: 0.5, y:0.6},
            glorp_type: 1,
            glorp_radius: 30.0,
            start_time: SystemTime::now(),
            highest_point: 0.0,                     //use later, maybe for high score?
            acceleration: 1.4,
            speed: 0.0,
            bounce_speed: -1.0,
            stop_duration_seconds_current: 0.0,
            stop_duration_seconds_max: 2.5,
            stopped: false,   
            ingame_height: 0.0, 
        };
            return ret;
    }

}