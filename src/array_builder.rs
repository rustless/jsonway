use serialize::json;
use serialize::json::{JsonArray, Json, ToJson};
use std::collections::TreeMap;

use object_builder::ObjectBuilder;

pub struct ArrayBuilder {
    array: JsonArray,
    pub null: bool,
    pub skip: bool,
    pub root: Option<String>
}

/// Use ArrayBuilder to produce JSON arrays
impl ArrayBuilder {

    pub fn new() -> ArrayBuilder {
        ArrayBuilder { 
            array: vec![], 
            null: false,
            skip: false,
            root: None
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(array: Json) -> Option<ArrayBuilder> {
        match array {
            json::Array(array) => Some(ArrayBuilder { 
                array: array, 
                null: false,
                skip: false,
                root: None
            }),
            _ => None
        }
    }

    /// Create new ArrayBuilder, pass it to closure as mutable ref and return.
    pub fn build(builder: |&mut ArrayBuilder|) -> ArrayBuilder {
        let mut bldr = ArrayBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    /// Push JSON value to array.
    pub fn push_json(&mut self, value: Json) {
        self.array.push(value);
    }

    /// Create new array and push it.
    pub fn array(&mut self, builder: |&mut ArrayBuilder|) {
        self.push(ArrayBuilder::build(builder).unwrap());
    }

    /// Create new object and push it
    pub fn object(&mut self, builder: |&mut ObjectBuilder|) {
        self.push(ObjectBuilder::build(builder).unwrap());
    }

    /// It you call `null`, this array will be converted to null when converting
    /// to raw JSON value.
    pub fn null(&mut self) {
        self.null = true;
    }

    /// It you call `skip`, this array will be skipped.
    pub fn skip(&mut self) {
        self.skip = true;
    }

    // Set custom root for result Json object
    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
    }

    pub fn has_root(&mut self) -> bool {
        self.root.is_some()
    }

    /// Move out internal JSON value.
    pub fn unwrap(self) -> Json {
        if self.root.is_some() {
            let mut obj = TreeMap::new();
            let root = self.root.as_ref().unwrap().to_string();
            let self_json = self.unwrap_internal();
            obj.insert(root, self_json);
            json::Object(obj)
        } else {
            self.unwrap_internal()
        }
    }

    /// Move out internal JSON value.
    #[inline]
    fn unwrap_internal(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::Array(self.array)
        }
    }
}

impl<T: ToJson> ArrayBuilder {
    /// Push to array something that can be converted to JSON.
    pub fn push(&mut self, value: T) {
        self.push_json(value.to_json());
    }
}

impl<A, T: Iterator<A>> ArrayBuilder {
    /// Fill this array by objects builded from iterator.
    pub fn objects(&mut self, iter: &mut T, func: |A, &mut ObjectBuilder|) {
        for a in *iter {
            let mut bldr = ObjectBuilder::new();
            func(a, &mut bldr);
            if !bldr.skip {
                self.push(bldr.unwrap())
            }    
        }
    }

    // Fill this array by arrays builded from iterator.
    pub fn arrays(&mut self, iter: &mut T, func: |A, &mut ArrayBuilder|) {
        for a in *iter {
            let mut bldr = ArrayBuilder::new();
            func(a, &mut bldr);
            if !bldr.skip {
                self.push(bldr.unwrap())
            }    
        }
    }

    /// Fill this array by JSON values builded from iterator.
    pub fn map(&mut self, iter: &mut T, func: |A| -> Json) {
        for a in *iter {
            self.push(func(a))      
        }
    }
}

impl ToJson for ArrayBuilder {
    /// Copy self to new JSON instance.
    fn to_json(&self) -> Json {
         if self.null { json::Null } else { self.array.to_json() }
    }
}
