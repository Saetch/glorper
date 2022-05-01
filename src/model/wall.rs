use super::{glorper_object::Pos, field_thing::FieldThing};

pub struct Wall{
    pub(crate) width : f64,
    pub(crate) height: f64,
    pub(crate) position: Pos,
 }

 impl FieldThing for Wall{
    fn update(&mut self){

    }

    fn get_pos(&self)-> Pos {
        self.position
    }
    
    fn is_in_proximity_to(&self, pos : Pos) -> bool{
        return f64::abs(self.position.x - pos.x) < self.width || f64::abs(self.position.y - pos.y) < self.width;
    }

    fn get_type(&self) -> &str{
        return "Wall";
    }
 }