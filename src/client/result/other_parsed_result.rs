use std::any::Any;
use std::fmt::Debug;

use super::ParsedRXingResult;

#[derive(Debug)]
pub struct OtherParsedResult {
    data: Box<dyn DynEq>,
}

impl ParsedRXingResult for OtherParsedResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        super::ParsedRXingResultType::Other
    }

    fn getDisplayRXingResult(&self) -> String {
        format!("{:?}", self.data)
    }
}

pub trait DynEq: Any + Debug {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

// Every 'static type that implements PartialEq gets this for free
impl<T: Any + PartialEq + Debug> DynEq for T {
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>().map_or(false, |o| self == o)
    }
}

impl OtherParsedResult {
    pub fn new<T: DynEq>(data: T) -> Self {
        Self {
            data: Box::new(data),
        }
    }

    /// Accepts an already-boxed value
    pub fn from_boxed(data: Box<dyn DynEq>) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &dyn Any {
        &*self.data // upcast dyn DynEq -> dyn Any (Rust 1.86+)
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.get_data().downcast_ref::<T>()
    }
}

/// This always returns false, any cannot be equal
impl PartialEq for OtherParsedResult {
    fn eq(&self, other: &Self) -> bool {
        self.data.dyn_eq(&*other.data)
    }
}

impl Eq for OtherParsedResult {}
