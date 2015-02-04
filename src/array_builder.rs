use serialize::json;
use std::collections;

use object_builder;

pub struct ArrayBuilder {
    pub array: json::Array,
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
    pub fn from_json(array: json::Json) -> Option<ArrayBuilder> {
        match array {
            json::Json::Array(array) => Some(ArrayBuilder { 
                array: array, 
                null: false,
                skip: false,
                root: None
            }),
            _ => None
        }
    }

    /// Create new ArrayBuilder, pass it to closure as mutable ref and return.
    pub fn build<F>(builder: F) -> ArrayBuilder where F: FnOnce(&mut ArrayBuilder) {
        let mut bldr = ArrayBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    /// Push JSON value to array.
    pub fn push_json(&mut self, value: json::Json) {
        self.array.push(value);
    }

    /// Create new array and push it.
    pub fn array<F>(&mut self, builder: F) where F: FnOnce(&mut ArrayBuilder) {
        self.push(ArrayBuilder::build(builder).unwrap());
    }

    /// Create new object and push it
    pub fn object<F>(&mut self, builder: F) where F: FnOnce(&mut object_builder::ObjectBuilder) {
        self.push(object_builder::ObjectBuilder::build(builder).unwrap());
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

    // Set custom root for result json::Json object
    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
    }

    pub fn has_root(&mut self) -> bool {
        self.root.is_some()
    }

    /// Move out internal JSON value.
    pub fn unwrap(self) -> json::Json {
        if self.root.is_some() {
            let mut obj = collections::BTreeMap::new();
            let root = self.root.as_ref().unwrap().to_string();
            let self_json = self.unwrap_internal();
            obj.insert(root, self_json);
            json::Json::Object(obj)
        } else {
            self.unwrap_internal()
        }
    }

    /// Move out internal JSON value.
    #[inline]
    fn unwrap_internal(self) -> json::Json {
        if self.null {
            json::Json::Null
        } else {
            json::Json::Array(self.array)
        }
    }
}

impl ArrayBuilder {
    /// Push to array something that can be converted to JSON.
    pub fn push<T: json::ToJson>(&mut self, value: T) {
        self.push_json(value.to_json());
    }
}

impl ArrayBuilder {

    /// Fill this array by objects builded from iterator.
    pub fn objects<A, T: Iterator<Item=A>, F>(&mut self, iter: T, func: F) where F: Fn(A, &mut object_builder::ObjectBuilder) {
        for a in iter {
            let mut bldr = object_builder::ObjectBuilder::new();
            func(a, &mut bldr);
            if !bldr.skip {
                self.push(bldr.unwrap())
            }    
        }
    }

    // Fill this array by arrays builded from iterator.
    pub fn arrays<A, T: Iterator<Item=A>, F>(&mut self, iter: T, func: F) where F: Fn(A, &mut ArrayBuilder) {
        for a in iter {
            let mut bldr = ArrayBuilder::new();
            func(a, &mut bldr);
            if !bldr.skip {
                self.push(bldr.unwrap())
            }    
        }
    }

    /// Fill this array by JSON values builded from iterator.
    pub fn map<A, T: Iterator<Item=A>, F>(&mut self, iter: T, func: F) where F: Fn(A) -> json::Json {
        for a in iter {
            self.push(func(a))      
        }
    }
}

impl json::ToJson for ArrayBuilder {
    /// Copy self to new JSON instance.
    fn to_json(&self) -> json::Json {
         if self.null { json::Json::Null } else { self.array.to_json() }
    }
}
