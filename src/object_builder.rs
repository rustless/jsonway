use std::collections::BTreeMap;
use collections::string::ToString;
use serialize::json::{Json, Object, ToJson};

use array_builder::ArrayBuilder;

pub struct ObjectBuilder {
    object: Object,
    pub null: bool,
    pub skip: bool,
    pub root: Option<String>
}

/// ObjectBuilder is used to produce JSON objects
#[allow(unstable)]
impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { 
            object: BTreeMap::new(), 
            null: false,
            skip: false,
            root: None
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(object: Json) -> Option<ObjectBuilder> {
        match object {
            Json::Object(object) => Some(ObjectBuilder { 
                object: object, 
                null: false,
                skip: false,
                root: None
            }),
            _ => None
        }
    }

    /// Create new builder, pass it to closure as mutable ref and return.
    pub fn build<F>(builder: F) -> ObjectBuilder where F: Fn(&mut ObjectBuilder) {
        let mut bldr = ObjectBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    /// It you call `null`, this object will be converted to null.
    pub fn null(&mut self) {
        self.null = true;
    }

    /// It you call `skip`, this object will be skipped.
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
            let mut obj = BTreeMap::new();
            let root = self.root.as_ref().unwrap().to_string();
            let self_json = self.unwrap_internal();
            obj.insert(root, self_json);
            Json::Object(obj)
        } else {
            self.unwrap_internal()
        }
    }

    #[inline]
    fn unwrap_internal(self) -> Json {
        if self.null {
            Json::Null
        } else {
            Json::Object(self.object)    
        }
    }
}

impl ObjectBuilder {
    /// Set object's `name` field with something that can be
    /// converted to Json value.
    pub fn set<V: ToJson, N: ToString>(&mut self, name: N, value: V) {
        self.set_json(name, value.to_json());
    }

    /// Stub for future use
    pub fn call<V: ToJson, N: ToString>(&mut self, name: N, value: V) {
        self.set(name, value);
    }
}

#[allow(unstable)]
impl ObjectBuilder {
    /// Set object's `name` field with raw Json value.
    pub fn set_json<N: ToString>(&mut self, name: N, value: Json) {
        self.object.insert(name.to_string(), value);
    }

    /// Build new array and set object's `name` field with it.
    pub fn array<N: ToString, F>(&mut self, name: N, builder: F) where F: Fn(&mut ArrayBuilder) {
        self.set(name, ArrayBuilder::build(builder).unwrap());
    }

    /// Build new object and set object's `name` field with it.
    pub fn object<N: ToString, F>(&mut self, name: N, builder: F) where F: Fn(&mut ObjectBuilder) {
        self.set(name, ObjectBuilder::build(builder).unwrap());
    }
}

impl ToJson for ObjectBuilder {
    /// Copy self to new JSON instance.
    fn to_json(&self) -> Json {
        if self.null { Json::Null } else { self.object.to_json() }
    }
}

