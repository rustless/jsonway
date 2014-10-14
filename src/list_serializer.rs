use serialize::json::Json;

use object_builder::ObjectBuilder;
use list_builder::ListBuilder;

pub trait ListSerializer {

    fn build(&self, &mut ListBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    #[inline]
    fn meta(&self) -> Option<ObjectBuilder> {
        None
    }

    fn serialize(&mut self) -> Json {
        let mut bldr = ListBuilder::new();

        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }

        self.build(&mut bldr);

        match self.meta() {
            Some(meta) => {
                let mut meta_bldr = if bldr.has_root() {
                    ObjectBuilder::from_json(bldr.unwrap()).unwrap()
                } else {
                    let mut meta_bldr = ObjectBuilder::new();
                    meta_bldr.set("data", bldr.unwrap());
                    meta_bldr
                };
                meta_bldr.set_json("meta", meta.unwrap());
                meta_bldr.unwrap()    
            },
            None => {
                bldr.unwrap()    
            }
        }
    }
}