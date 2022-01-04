use piston::UpdateArgs;

pub struct Model{

}


impl Model {
    
    pub fn update(&self, args : &UpdateArgs){
        //DO something
    }

    pub fn tickle_glorper(&self){
        println!("Someone pressed 'H', this tickles!");
    }
}