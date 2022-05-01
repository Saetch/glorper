use super::glorper_object::Pos;

pub trait FieldThing : Send + Sync{
    fn update(&mut self);

    fn get_pos(&self) -> Pos;

    fn is_in_proximity_to(&self, position : Pos) -> bool;

    fn get_type(&self) -> &str;
}