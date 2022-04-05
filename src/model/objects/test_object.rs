use crate::model::glorper_object::GlorperObject;

pub struct TestObject{
    pos : (f32, f32),
    texture_id : i32,
}

impl TestObject{
    pub fn new() -> Self{
        TestObject{
            pos: (0.0f32, 0.0f32),
            texture_id : -1,
        }
    }
}

impl GlorperObject for TestObject {
    fn get_pos(&self) -> crate::model::glorper_object::Pos {
        todo!()
    }

    fn get_texture_id(&self) -> i16 {
        todo!()
    }

    fn get_interaction_type(&self) -> u16 {
        todo!()
    }
}