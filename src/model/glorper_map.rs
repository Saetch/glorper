use crate::constants::{FIELDWIDTH, FIELDHEIGHT};
use crate::model::glorper_field_tile::GlorperFieldTile;
use super::{glorper_object::GlorperObject};

pub struct  GlorperMap{
    pub(crate)  map : [[GlorperFieldTile; FIELDWIDTH] ; FIELDHEIGHT],
}



impl GlorperMap{
    pub fn get_object_at(&self, x : usize, y : usize){
        
    }
}