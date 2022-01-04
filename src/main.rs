use std::{sync::{RwLock, Arc}, time::Instant};

use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::{WindowSettings, Events, EventSettings, RenderEvent, UpdateEvent, ButtonEvent};


use crate::{controller::controller::{Controller}, constants::{FIELDWIDTH, FIELDHEIGHT}, model::model::Model, view::view::View};


extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod constants;
mod model;
mod view;
mod controller;





fn main() {

   


    
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("glorper!", [800, 600])
        .graphics_api(opengl)
        .build()
        .unwrap();

        //an Arc only needs to be mutable if the reference is meant to point to another place later
        let running = Arc::new(RwLock::new(true));


         //This is the event buffer, it gets constantly filled if an event occurs to the window and is read in the game loop
        let mut events = Events::new(EventSettings::new());




        let model_arc = Arc::new(Model{});
        let cntrl = Controller{model: model_arc.clone()};
        let mut view = View{gl: GlGraphics::new(opengl)};


        //create the channel for communication between threads
        let (modelchannelsender, modelchannelreceiver) = flume::unbounded();
        let (controllerchannelsender, controllerchannelreceiver) = flume::unbounded();



        /*
            create 2 threads. One for the controller, one for the model

        */


        let running_thread = running.clone();

        let controller_thread = std::thread::spawn(move || {
            while *running_thread.read().unwrap() {
                let arg_guard = controllerchannelreceiver.recv();
                if !arg_guard.is_err(){
                    let args = arg_guard.unwrap();
                    cntrl.compute_input(&args);
                }
            }
        });

        let running_thread = running.clone();    //since it was moved into controller_thread, the running_thread variable is gone and needs to be declared again
        
        let model_thread = std::thread::spawn(move || {
            while *running_thread.read().unwrap() {
                let arg_guard = modelchannelreceiver.recv();
                if !arg_guard.is_err(){
                    let args = arg_guard.unwrap();
                    model_arc.update(&args);                    //this only works directly if model is immutable. Otherwise an RwLock is needed. To keep it immutable, all variables that might change need to be synced (RwLock or Mutex)
                }
            }
        });





        /*
         * This threads game loop (Rendering)
         */
        println!("Press H to tickle Glorper!\n\n");
        let mut start = Instant::now();                         
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
    
                view.render(&args);
                let elapsed = start.elapsed();
                //println!("Elapsed time: {}ms", elapsed.as_millis());              //This gets called 60 times per second (60fps), if the elapsed time shown is >17, the fps dropped
                start = Instant::now();  
                continue;                                      //SKIP the other possible updateArgs, because only one can be valid
            }
    
            if let Some(args) = e.update_args() {
                let ret = modelchannelsender.try_send(args);
                if ret.is_err(){
                    println!("Could not send args to model thread!");           //if the app still works when this occurs, this might mean that the buffer on model side is full and it might be behind schedule
                }
                continue;                                     //SKIP the other possible updateArgs, because only one can be validoo
            }
    
            if let Some(args) = e.button_args(){
                let ret = controllerchannelsender.try_send(args);
                if ret.is_err(){
                    println!("Could not send args to controller thread!");      //if the app still works when this occurs, this might mean that the buffer on controller side is full and it might be behind schedule
                }
                continue;                                     //SKIP the other possible updateArgs, because only one can be validoo
            }
        }



        //sync threads

        drop(modelchannelsender);
        drop(controllerchannelsender);
        let mut running_variable = running.write().unwrap();    
        *running_variable = false;                                  //make the loops of the threads stop!
        drop(running_variable);


        model_thread.join().unwrap();
        controller_thread.join().unwrap();
        println!("I stopped like I was supposed to!");



}
