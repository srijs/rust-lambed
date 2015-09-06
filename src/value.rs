use std::any::{TypeId, Any};
use std::fmt::Debug;

pub trait AsValue: Any + Debug {
    fn get_any(&self) -> &Any;
    fn any_eq(&self, other: &Any) -> bool;
}

impl<T: Any + Eq + Debug> AsValue for T {
    fn get_any(&self) -> &Any {
        self
    }
    fn any_eq(&self, other: &Any) -> bool {
        match Any::downcast_ref(other) {
            None => false,
            Some(other_cast) => self == other_cast
        }
    }
}

#[derive (Debug)]
pub struct Value {
    pub typ: TypeId,
    pub val: Box<AsValue>
}

impl Value {
    pub fn new<T: AsValue>(x: T) -> Value {
        Value {
            typ: TypeId::of::<T>(),
            val: Box::new(x)
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.val.any_eq((&*other.val).get_any())
    }
}

impl Eq for Value {}
