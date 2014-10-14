use std::collections::TreeMap;
use std::to_string::ToString;
use serialize::json;
use serialize::json::{Json, JsonObject, ToJson};

use list_builder::ListBuilder;

pub struct ObjectBuilder {
    object: JsonObject,
    pub null: bool,
    pub skip: bool,
    root: Option<String>
}

/// ObjectBuilder is used to produce JSON objects
impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { 
            object: TreeMap::new(), 
            null: false,
            skip: false,
            root: None
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(object: Json) -> Option<ObjectBuilder> {
        match object {
            json::Object(object) => Some(ObjectBuilder { 
                object: object, 
                null: false,
                skip: false,
                root: None
            }),
            _ => None
        }
    }

    /// Create new builder, pass it to closure as mutable ref and return.
    pub fn build(builder: |&mut ObjectBuilder|) -> ObjectBuilder {
        let mut bldr = ObjectBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    /// It you call `null`, this list will be converted to null.
    pub fn null(&mut self) {
        self.null = true;
    }

    /// It you call `skip`, this list will be skipped.
    pub fn skip(&mut self) {
        self.skip = true;
    }

    // Set custom root for result Json object
    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
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

    #[inline]
    fn unwrap_internal(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::Object(self.object)    
        }
    }
}

impl<V: ToJson, N: ToString> ObjectBuilder {
    /// Set object's `name` field with something that can be
    /// converted to Json value.
    pub fn set(&mut self, name: N, value: V) {
        self.set_json(name.to_string(), value.to_json());
    }

    /// Stub for future use
    pub fn call(&mut self, name: N, value: V) {
        self.set(name, value);
    }
}

impl<N: ToString> ObjectBuilder {
    /// Set object's `name` field with raw Json value.
    pub fn set_json(&mut self, name: N, value: Json) {
        self.object.insert(name.to_string(), value);
    }

    /// Build new list and set object's `name` field with it.
    pub fn list(&mut self, name: N, builder: |&mut ListBuilder|) {
        self.set(name, ListBuilder::build(builder).unwrap());
    }

    /// Build new object and set object's `name` field with it.
    pub fn object(&mut self, name: N, builder: |&mut ObjectBuilder|) {
        self.set(name, ObjectBuilder::build(builder).unwrap());
    }
}

impl ToJson for ObjectBuilder {
    /// Copy self to new JSON instance.
    fn to_json(&self) -> Json {
        if self.null { json::Null } else { self.object.to_json() }
    }
}

