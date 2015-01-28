#![crate_name = "jsonway"]
#![crate_type = "rlib"]
#![allow(unstable)]
#![deny(warnings)]
#![deny(bad_style)]

extern crate "rustc-serialize" as serialize;
extern crate collections;

pub use mutable_json::MutableJson;
pub use object_builder::ObjectBuilder;
pub use array_builder::ArrayBuilder;
pub use serializer::{Serializer, ObjectSerializer, ObjectScopeSerializer};
pub use array_serializer::ArraySerializer;

pub mod array_builder;
pub mod object_builder;
pub mod mutable_json;
pub mod serializer;
pub mod array_serializer;

/// ```rust
/// let json = jsonway::object(|json| {
///     json.set("first_name", "Luke".to_string()); 
///     json.set("last_name", "Skywalker".to_string());
///
///     json.object("info", |json| {
///         json.set("homeworld", "Tatooine".to_string());
///         json.set("born", "19 BBY".to_string());
///         json.set("died", "Between 45 ABY and 137 ABY".to_string());
///     });
///
///     json.array("masters", |json| {
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
/// assert!(json.find("masters").unwrap().is_array());
/// ```

/// Create and return new ListBuilder
pub fn array<F>(builder: F) -> ArrayBuilder where F: FnOnce(&mut ArrayBuilder) {
    ArrayBuilder::build(builder)
}    

/// Create and return new ObjectBuilder
pub fn object<F>(builder: F) -> ObjectBuilder where F: FnOnce(&mut ObjectBuilder) {
    ObjectBuilder::build(builder)
}
