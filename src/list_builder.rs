use serialize::json;
use serialize::json::{JsonList, Json, ToJson};
use std::collections::TreeMap;

use object_builder::ObjectBuilder;

pub struct ListBuilder {
    list: JsonList,
    pub null: bool,
    pub skip: bool,
    root: Option<String>
}

/// Use ListBuilder to produce JSON arrays
impl ListBuilder {

    pub fn new() -> ListBuilder {
        ListBuilder { 
            list: vec![], 
            null: false,
            skip: false,
            root: None
        }
    }

    /// Initialize builder with initial value.
    pub fn from_json(list: Json) -> Option<ListBuilder> {
        match list {
            json::List(list) => Some(ListBuilder { 
                list: list, 
                null: false,
                skip: false,
                root: None
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

    // Set custom root for result Json object
    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
    }

    /// Move out internal JSON value.
    pub fn unwrap(self) -> Json {
        if self.root.is_some() {
            let mut obj = TreeMap::new();
            let root = self.root.as_ref().unwrap().to_string();
            let self_json = self.unwrap_internal();
            obj.insert(root, self_json);
            json::Object(obj)
        } else {
            self.unwrap_internal()
        }
    }

    /// Move out internal JSON value.
    #[inline]
    fn unwrap_internal(self) -> Json {
        if self.null {
            json::Null
        } else {
            json::List(self.list)
        }
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
