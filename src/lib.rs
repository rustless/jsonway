#![crate_name = "jsonway"]
#![comment = "JSON builder for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
// #![deny(warnings)]
// #![deny(bad_style)]
#![feature(macro_rules, phase, tuple_indexing, unboxed_closure_sugar)]

extern crate serialize;

pub use mutable_json::MutableJson;
pub use object_builder::ObjectBuilder;
pub use list_builder::ListBuilder;
pub use serializer::Serializer;
pub use list_serializer::ListSerializer;

pub mod list_builder;
pub mod object_builder;
pub mod mutable_json;
pub mod serializer;
pub mod list_serializer;

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
