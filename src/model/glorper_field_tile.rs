use std::sync::{RwLock, Arc};

use super::glorper_object::GlorperObject;

pub struct GlorperFieldTile{

    pub tile_type : i16,
    pub objects: Vec<Arc<RwLock<dyn GlorperObject>>>,
}