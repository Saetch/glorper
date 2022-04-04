pub trait GlorperObject : Sync{
    fn get_object_type(&self) -> String{
        return String::from("Type not yet implemented");
    }
}