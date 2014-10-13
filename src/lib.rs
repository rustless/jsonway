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

pub use mutable_json::MutableJson;

pub mod mutable_json;

pub struct ListBuilder {
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

    pub fn push_json(&mut self, value: Json) {
        self.list.push(value);
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

impl<T: ToJson> ListBuilder {
    pub fn push(&mut self, value: T) {
        self.push_json(value.to_json());
    }
}

impl<A, T: Iterator<A>> ListBuilder {
    pub fn objects(&mut self, iter: &mut T, func: |A, &mut ObjectBuilder|) {
        let mut stop = false;
        while stop {
            let a = iter.next();
            if a.is_some() {
                let mut bldr = ObjectBuilder::new();
                func(a.unwrap(), &mut bldr);
                self.push(bldr.move_to_json())
            } else {
                stop = true;
            }
        }
    }

    pub fn lists(&mut self, iter: &mut T, func: |A, &mut ListBuilder|) {
        let mut stop = false;
        while stop {
            let a = iter.next();
            if a.is_some() {
                let mut bldr = ListBuilder::new();
                func(a.unwrap(), &mut bldr);
                self.push(bldr.move_to_json())
            } else {
                stop = true;
            }
        }
    }

    pub fn map(&mut self, iter: &mut T, func: |A| -> Json) {
        let mut stop = false;
        while stop {
            let a = iter.next();
            if a.is_some() {
                self.push(func(a.unwrap()))
            } else {
                stop = true;
            }
        }
    }
}

impl ToJson for ListBuilder {
    fn to_json(&self) -> Json {
         if self.null { json::Null } else { self.list.to_json() }
    }
}

pub struct ObjectBuilder {
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

    pub fn index(&mut self) -> bool {
        true    
    }
}

impl<V: ToJson, N: ToString> ObjectBuilder {
    pub fn set(&mut self, name: N, value: V) {
        self.set_json(name.to_string(), value.to_json());
    }

    pub fn call(&mut self, name: N, value: V) {
        self.set(name, value);
    }
}

impl<N: ToString> ObjectBuilder {
    pub fn set_json(&mut self, name: N, value: Json) {
        self.object.insert(name.to_string(), value);
    }

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
fn simple_object() {
    ObjectBuilder::build(|json| {
        json.set("first_name", "Luke".to_string()); 
        json.set("last_name", "Skywalker".to_string());  
    });
}

#[test]
fn simple_list() {
    ListBuilder::build(|json| {
        json.push("Luke".to_string()); 
        json.push("Skywalker".to_string());  
    });
}