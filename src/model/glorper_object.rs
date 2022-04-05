use crate::view::texture_object::TextureObject;

pub trait GlorperObject : Send + Sync{
    fn get_object_type(&self) -> String{
        return String::from("Type not yet implemented");
    }

    fn get_pos(&self) -> Pos;

    fn get_draw_references(&self) -> Vec<TextureObject>;

    fn count_draw_references(&self) -> u16;

    fn add_draw_reference(&self, a : &TextureObject) -> u16;

    fn get_interaction_type(&self) -> u16;

}

pub struct Pos {
    x : f32,
    y : f32,
}