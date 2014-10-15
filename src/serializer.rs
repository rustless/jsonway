use serialize::json::Json;

use object_builder::ObjectBuilder;

/// Provides functionality to create custom JSON presenters for your structs.
/// 
/// ## Example 
/// 
/// ```
/// use jsonway::{ObjectBuilder, Serializer};
///
/// struct Jedi {
///     name: String
/// }
///
/// struct JediSerializer<'a> {
///     jedi: &'a Jedi
/// }
///
/// impl<'a> Serializer for JediSerializer<'a> {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, json: &mut ObjectBuilder) {
///         json.set("name", self.jedi.name.to_string());
///     }
/// }
///
/// let jedi = Jedi { name: "Saes Rrogon".to_string() };
/// let json = JediSerializer{jedi: &jedi}.serialize();
///
/// assert_eq!(
///     json.find_path(&[
///         &"jedi".to_string(),
///         &"name".to_string(),
///     ]).unwrap().as_string().unwrap(), 
///     "Saes Rrogon"
/// )
/// ```
pub trait Serializer {

    fn build(&self, &mut ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self) -> Json {
        let mut bldr = ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(&mut bldr);

        bldr.unwrap()
    }
}

/// Provides functionality to create custom JSON presenters for your structs.
/// 
/// ## Example 
/// 
/// ```
/// use jsonway::{ObjectBuilder, ObjectSerializer};
///
/// struct Jedi {
///     name: String
/// }
///
/// struct JediSerializer;
///
/// impl ObjectSerializer<Jedi> for JediSerializer {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, jedi: &Jedi, json: &mut ObjectBuilder) {
///         json.set("name", jedi.name.to_string());
///     }
/// }
///
/// let jedi = Jedi { name: "Saes Rrogon".to_string() };
/// let json = JediSerializer.serialize(&jedi);
///
/// assert_eq!(
///     json.find_path(&[
///         &"jedi".to_string(),
///         &"name".to_string(),
///     ]).unwrap().as_string().unwrap(), 
///     "Saes Rrogon"
/// )
/// ```
pub trait ObjectSerializer<T> {

    fn build(&self, &T, &mut ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T) -> Json {
        let mut bldr = ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, &mut bldr);
        bldr.unwrap()
    }
}