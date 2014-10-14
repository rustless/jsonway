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
/// struct JediSerializer;
///
/// impl Serializer<Jedi> for JediSerializer {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, jedi: &Jedi, json: &mut ObjectBuilder) {
///         json.set("name", jedi.name.to_string());
///     }
/// }
///
/// let jedi = Jedi { name: "Saes Rrogon".to_string() };
/// let json = JediSerializer.serialize(&jedi).unwrap();
/// ```
pub trait Serializer<T> {

    fn build(&self, &T, &mut ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T) -> ObjectBuilder {
        let mut bldr = ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, &mut bldr);

        bldr
    }
}