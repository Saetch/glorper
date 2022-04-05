use crate::view::texture_object::TextureObject;

pub trait GlorperObject : Send + Sync{
    fn get_object_type(&self) -> String{
        return String::from("Type not yet implemented");
    }

    fn get_pos(&self) -> Pos;

    fn get_texture_id(&self) -> i16;

    fn get_interaction_type(&self) -> u16;


}

#[derive(Clone)]
pub struct Pos {
    pub(crate) x : f32,
    pub(crate) y : f32,
}