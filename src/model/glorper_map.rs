use crate::constants::{FIELDWIDTH, FIELDHEIGHT};

use super::glorper_object::GlorperObject;
pub struct  GlorperMap<'map>{
    pub(crate)  map : [[&'map dyn GlorperObject; FIELDWIDTH] ; FIELDHEIGHT],
}



impl GlorperMap<'_>{
    pub fn getObjectAt(&self, x : usize, y : usize){
        
    }
}