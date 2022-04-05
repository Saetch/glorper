use crate::{model::glorper_object::{GlorperObject, Pos}, view::texture_object::TextureObject};

pub struct TestObject{
    pos : Pos,
    texture_id : i32,
}

impl TestObject{
    pub fn new() -> Self{
        TestObject{
            pos: Pos { x: 0.0f32, y: 0.0f32 },
            texture_id : -1,
        }
    }
}

impl GlorperObject for TestObject {
    fn get_pos(&self) -> crate::model::glorper_object::Pos {
        return self.pos.clone();
    }

    #[inline(always)]
    fn get_texture_id(&self) -> i16 {
        return TextureObject::id_test();
    }

    fn get_interaction_type(&self) -> u16 {
        return 0;
    }

    fn get_object_type(&self) -> String{
        return String::from("Type not yet implemented");
    }
}