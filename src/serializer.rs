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
///         "jedi",
///         "name",
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
///         "jedi",
///         "name",
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

/// Provides functionality to create custom JSON presenters for your structs.
/// 
/// ## Example 
/// 
/// ```rust
/// use jsonway::{ObjectBuilder, ObjectScopeSerializer};
/// 
/// struct User {
///     id: uint,
///     is_admin: bool
/// }
/// 
/// struct Jedi {
///     name: String,
///     secret: String
/// }
/// 
/// struct JediSerializer;
/// 
/// impl ObjectScopeSerializer<Jedi, User> for JediSerializer {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, jedi: &Jedi, current_user: &User, json: &mut ObjectBuilder) {
///         json.set("name", jedi.name.to_string());
/// 
///         if current_user.is_admin {
///             json.set("secret", jedi.secret.to_string());
///         }
///     }
/// }
/// 
/// let jedi = Jedi { 
///     name: "Palpatine".to_string(), 
///     secret: "Dark side".to_string() 
/// };
///
/// let current_user = User { id: 1, is_admin: true };
/// let json = JediSerializer.serialize(&jedi, &current_user);
///
/// assert_eq!(
///     json.find_path(&[
///         "jedi",
///         "name",
///     ]).unwrap().as_string().unwrap(), 
///     "Palpatine"
/// )
///
/// assert_eq!(
///     json.find_path(&[
///         "jedi",
///         "secret",
///     ]).unwrap().as_string().unwrap(), 
///     "Dark side"
/// )
///
/// ```
pub trait ObjectScopeSerializer<T, S> {

    fn build(&self, &T, &S, &mut ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T, scope: &S) -> Json {
        let mut bldr = ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, scope, &mut bldr);
        bldr.unwrap()
    }
}