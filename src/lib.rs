#![crate_name = "jsonway"]
#![comment = "JSON builder for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
// #![deny(warnings)]
// #![deny(bad_style)]
#![feature(macro_rules, phase, tuple_indexing)]

extern crate serialize;

use std::collections::TreeMap;
use std::to_string::ToString;
use serialize::json;
use serialize::json::{JsonList, Json, JsonObject, ToJson};

pub mod mutable_json;

trait Builder {

}

struct ListBuilder {
    list: JsonList,
    null: bool
}

impl ListBuilder {

    pub fn new() -> ListBuilder {
        ListBuilder { list: vec![], null: false }
    }

    pub fn from_json(list: Json) -> Option<ListBuilder> {
        match list {
            json::List(list) => Some(ListBuilder { list: list, null: false }),
            _ => None
        }
    }

    pub fn build(builder: |&mut ListBuilder|) -> ListBuilder {
        let mut bldr = ListBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    pub fn move_to_json(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::List(self.list)
        }
    }
}

impl<T: ToJson> ListBuilder {
    pub fn push(&mut self, value: T) {
        self.list.push(value.to_json());
    }

    pub fn list(&mut self, builder: |&mut ListBuilder|) {
        self.push(ListBuilder::build(builder).move_to_json());
    }

    pub fn object(&mut self, builder: |&mut ObjectBuilder|) {
        self.push(ObjectBuilder::build(builder).move_to_json());
    }

    pub fn null(&mut self) {
        self.null = true;
    }
}

impl ToJson for ListBuilder {
    fn to_json(&self) -> Json {
         if self.null { json::Null } else { self.list.to_json() }
    }
}

struct ObjectBuilder {
    object: JsonObject,
    null: bool
}

impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { object: TreeMap::new(), null: false }
    }

    pub fn from_json(object: Json) -> Option<ObjectBuilder> {
        match object {
            json::Object(object) => Some(ObjectBuilder { object: object, null: false }),
            _ => None
        }
    }

    pub fn build(builder: |&mut ObjectBuilder|) -> ObjectBuilder {
        let mut bldr = ObjectBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    pub fn move_to_json(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::Object(self.object)    
        }
    }

    pub fn null(&mut self) {
        self.null = true;
    }
}

impl<V: ToJson, N: ToString> ObjectBuilder {
    pub fn set(&mut self, name: N, value: V) {
        self.object.insert(name.to_string(), value.to_json());
    }
}

impl<N: ToString> ObjectBuilder {
    pub fn list(&mut self, name: N, builder: |&mut ListBuilder|) {
        self.set(name, ListBuilder::build(builder).move_to_json());
    }

    pub fn object(&mut self, name: N, builder: |&mut ObjectBuilder|) {
        self.set(name, ObjectBuilder::build(builder).move_to_json());
    }
}

impl ToJson for ObjectBuilder {
    fn to_json(&self) -> Json {
        if self.null { json::Null } else { self.object.to_json() }
    }
}

#[test]
fn test() {
    let b = ListBuilder::new();
}