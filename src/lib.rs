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
    null: bool,
    skip: bool
}

/// Use ListBuilder to produce JSON arrays
impl ListBuilder {

    pub fn new() -> ListBuilder {
        ListBuilder { 
            list: vec![], 
            null: false,
            skip: false
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(list: Json) -> Option<ListBuilder> {
        match list {
            json::List(list) => Some(ListBuilder { 
                list: list, 
                null: false,
                skip: false
            }),
            _ => None
        }
    }

    /// Create new ListBuilder, pass it to closure as mutable ref and return.
    pub fn build(builder: |&mut ListBuilder|) -> ListBuilder {
        let mut bldr = ListBuilder::new();
        builder(&mut bldr);  
        
        bldr 
    }

    /// Move out internal JSON value.
    pub fn unwrap(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::List(self.list)
        }
    }

    /// Push JSON value to list.
    pub fn push_json(&mut self, value: Json) {
        self.list.push(value);
    }

    /// Create new list and push it.
    pub fn list(&mut self, builder: |&mut ListBuilder|) {
        self.push(ListBuilder::build(builder).unwrap());
    }

    /// Create new object and push it
    pub fn object(&mut self, builder: |&mut ObjectBuilder|) {
        self.push(ObjectBuilder::build(builder).unwrap());
    }

    /// It you call `null`, this list will be converted to null when converting
    /// to raw JSON value.
    pub fn null(&mut self) {
        self.null = true;
    }

    /// It you call `skip`, this list will be skipped.
    pub fn skip(&mut self) {
        self.skip = true;
    }
}

impl<T: ToJson> ListBuilder {
    /// Push to list something that can be converted to JSON.
    pub fn push(&mut self, value: T) {
        self.push_json(value.to_json());
    }
}

impl<A, T: Iterator<A>> ListBuilder {
    /// Fill this list by objects builded from iterator.
    pub fn objects(&mut self, iter: &mut T, func: |A, &mut ObjectBuilder|) {
        let mut stop = false;
        while !stop {
            let a = iter.next();
            if a.is_some() {
                let mut bldr = ObjectBuilder::new();
                func(a.unwrap(), &mut bldr);
                if !bldr.skip {
                    self.push(bldr.unwrap())
                }
            } else {
                stop = true;
            }
        }
    }

    // Fill this list by lists builded from iterator.
    pub fn lists(&mut self, iter: &mut T, func: |A, &mut ListBuilder|) {
        let mut stop = false;
        while !stop {
            let a = iter.next();
            if a.is_some() {
                let mut bldr = ListBuilder::new();
                func(a.unwrap(), &mut bldr);
                if !bldr.skip {
                    self.push(bldr.unwrap())
                }
            } else {
                stop = true;
            }
        }
    }

    /// Fill this list by JSON values builded from iterator.
    pub fn map(&mut self, iter: &mut T, func: |A| -> Json) {
        let mut stop = false;
        while !stop {
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
    /// Copy self to new JSON instance.
    fn to_json(&self) -> Json {
         if self.null { json::Null } else { self.list.to_json() }
    }
}

pub struct ObjectBuilder {
    object: JsonObject,
    null: bool,
    skip: bool
}


/// ListBuilder is used to produce JSON arrays
impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { 
            object: TreeMap::new(), 
            null: false,
            skip: false
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(object: Json) -> Option<ObjectBuilder> {
        match object {
            json::Object(object) => Some(ObjectBuilder { 
                object: object, 
                null: false,
                skip: false
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

    /// Move out internal JSON value.
    pub fn unwrap(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::Object(self.object)    
        }
    }

    /// It you call `null`, this list will be converted to null.
    pub fn null(&mut self) {
        self.null = true;
    }

    /// It you call `skip`, this list will be skipped.
    pub fn skip(&mut self) {
        self.skip = true;
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

pub struct JsonWay;

impl JsonWay {
    /// Create and return new ListBuilder
    pub fn list(builder: |&mut ListBuilder|) -> ListBuilder {
        ListBuilder::build(builder)
    }    

    /// Create and return new ObjectBuilder
    pub fn object(builder: |&mut ObjectBuilder|) -> ObjectBuilder {
        ObjectBuilder::build(builder)
    }
}

#[test]
fn simple() {
    let object = JsonWay::object(|json| {
        json.set("first_name", "Luke".to_string()); 
        json.set("last_name", "Skywalker".to_string());

        json.object("info", |json| {
            json.set("homeworld", "Tatooine".to_string());
            json.set("born", "19 BBY".to_string());
            json.set("died", "Between 45 ABY and 137 ABY".to_string());
        });

        json.list("masters", |json| {
            json.push("Obi-Wan Kenobi".to_string());
            json.push("Yoda".to_string());
            json.push("Joruus C'baoth (Briefly)".to_string());
            json.push("Darth Sidious (Briefly)".to_string());
        });
    });

    println!("{}", object.unwrap().to_pretty_str());

    // uncomment to dump
    // fail!("");

}

#[test]
fn iterations() {

    #[deriving(Show)]
    enum Side {
        Light,
        Dark
    }

    struct Jedi {
        name: String,
        side: Side
    }

    let jedi = vec![
        Jedi { name: "Saes Rrogon".to_string(), side: Dark },
        Jedi { name: "Qui-Gon Jinn".to_string(), side: Light },
        Jedi { name: "Obi-Wan Kenobi".to_string(), side: Light }
    ];

    let light_jedi_objects_list = JsonWay::list(|json| {
        json.objects(&mut jedi.iter(), |jedi, json| {
            match jedi.side {
                Light => {
                    json.set("name".to_string(), jedi.name.to_string());
                    json.set("side".to_string(), jedi.side.to_string());
                },
                Dark => json.skip()
            }
        })
    });

    println!("{}", light_jedi_objects_list.unwrap().to_pretty_str());

    let light_jedi_tuple_list = JsonWay::list(|json| {
        json.lists(&mut jedi.iter(), |jedi, json| {
            match jedi.side {
                Light => {
                    json.push(jedi.name.to_string());
                    json.push(jedi.side.to_string());
                },
                Dark => json.skip()
            }
        })
    });

    println!("{}", light_jedi_tuple_list.unwrap().to_pretty_str());

    // uncomment to dump
    // fail!("");

}
