use serialize::json::{Json, Object, Array};

pub trait MutableJson {
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut Object>;
    fn as_array_mut<'a>(&'a mut self) -> Option<&'a mut Array>;
}

impl MutableJson for Json {
    
    /// If the Json value is an Object, returns the associated TreeMap.
    /// Returns None otherwise.
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut Object> {
        match self {
            &Json::Object(ref mut map) => Some(&mut*map),
            _ => None
        }
    }

    fn as_array_mut<'a>(&'a mut self) -> Option<&'a mut Array> {
        match self {
            &Json::Array(ref mut array) => Some(&mut *array),
            _ => None
        }
    }

}