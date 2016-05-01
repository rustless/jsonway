use std::collections;
use serialize::json;

use array_builder;

pub struct ObjectBuilder {
    pub object: json::Object,
    pub null: bool,
    pub skip: bool,
    pub root: Option<String>
}

/// ObjectBuilder is used to produce JSON objects
impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder {
            object: collections::BTreeMap::new(),
            null: false,
            skip: false,
            root: None
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(object: json::Json) -> Option<ObjectBuilder> {
        match object {
            json::Json::Object(object) => Some(ObjectBuilder {
                object: object,
                null: false,
                skip: false,
                root: None
            }),
            _ => None
        }
    }

    /// Create new builder, pass it to closure as mutable ref and return.
    pub fn build<F>(builder: F) -> ObjectBuilder where F: FnOnce(&mut ObjectBuilder) {
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

    #[inline]
    fn unwrap_internal(self) -> json::Json {
        if self.null {
            json::Json::Null
        } else {
            json::Json::Object(self.object)
        }
    }
}

impl ObjectBuilder {
    /// Set object's `name` field with something that can be
    /// converted to json::Json value.
    pub fn set<V: json::ToJson, N: ToString>(&mut self, name: N, value: V) {
        self.set_json(name, value.to_json());
    }

    /// Stub for future use
    pub fn call<V: json::ToJson, N: ToString>(&mut self, name: N, value: V) {
        self.set(name, value);
    }
}

impl ObjectBuilder {
    /// Set object's `name` field with raw json::Json value.
    pub fn set_json<N: ToString>(&mut self, name: N, value: json::Json) {
        self.object.insert(name.to_string(), value);
    }

    /// Build new array and set object's `name` field with it.
    pub fn array<N: ToString, F>(&mut self, name: N, builder: F) where F: FnOnce(&mut array_builder::ArrayBuilder) {
        self.set(name, array_builder::ArrayBuilder::build(builder).unwrap());
    }

    /// Build new object and set object's `name` field with it.
    pub fn object<N: ToString, F>(&mut self, name: N, builder: F) where F: FnOnce(&mut ObjectBuilder) {
        self.set(name, ObjectBuilder::build(builder).unwrap());
    }
}

impl json::ToJson for ObjectBuilder {
    /// Copy self to new JSON instance.
    fn to_json(&self) -> json::Json {
        if self.null { json::Json::Null } else { self.object.to_json() }
    }
}

