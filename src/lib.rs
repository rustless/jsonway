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
pub use serializer::{Serializer, ObjectSerializer, ObjectScopeSerializer};
pub use list_serializer::ListSerializer;

pub mod list_builder;
pub mod object_builder;
pub mod mutable_json;
pub mod serializer;
pub mod list_serializer;

pub struct JsonWay;

/// ```rust
/// use jsonway::JsonWay;
///
/// let json = JsonWay::object(|json| {
///     json.set("first_name", "Luke".to_string()); 
///     json.set("last_name", "Skywalker".to_string());
///
///     json.object("info", |json| {
///         json.set("homeworld", "Tatooine".to_string());
///         json.set("born", "19 BBY".to_string());
///         json.set("died", "Between 45 ABY and 137 ABY".to_string());
///     });
///
///     json.list("masters", |json| {
///         json.push("Obi-Wan Kenobi".to_string());
///         json.push("Yoda".to_string());
///         json.push("Joruus C'baoth (Briefly)".to_string());
///         json.push("Darth Sidious (Briefly)".to_string());
///     });
/// }).unwrap();
///
/// assert_eq!(json.find("first_name").unwrap().as_string().unwrap(), "Luke");
/// assert_eq!(json.find("last_name").unwrap().as_string().unwrap(), "Skywalker");
///
/// assert!(json.find("info").unwrap().is_object());
/// assert!(json.find("masters").unwrap().is_list());
/// ```
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
