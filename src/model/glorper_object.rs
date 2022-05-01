use crate::view::texture_object::TextureObject;

//TODO: complete this if intended to add more complex objects!
pub trait GlorperObject : Send + Sync{
    fn get_object_type(&self) -> String{
        return String::from("Type not yet implemented");
    }

    fn get_pos(&self) -> Pos;

    fn get_texture_id(&self) -> i16;

    fn get_interaction_type(&self) -> u16;


}

#[derive(Clone, Copy)]
pub struct Pos {
    pub(crate) x : f64,
    pub(crate) y : f64,
}

impl Pos {
    pub fn add_x_y(&mut self, x: f64, y:f64){
        self.x+=x;
        self.y+=y;
    }
    pub fn add_x(&mut self, x:f64){
        self.x+=x;
    }

    pub fn add_y(&mut self, y:f64){
        self.y+=y;
    }
}