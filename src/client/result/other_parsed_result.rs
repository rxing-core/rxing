use std::any::Any;

use super::ParsedRXingResult;

#[derive(Debug)]
pub struct OtherParsedResult {
    data: Box<dyn Any>,
}

impl ParsedRXingResult for OtherParsedResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        super::ParsedRXingResultType::Other
    }

    fn getDisplayRXingResult(&self) -> String {
        format!("{:?}", self.data)
    }
}

impl OtherParsedResult {
    pub fn new(data: Box<dyn Any>) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &Box<dyn Any> {
        &self.data
    }
}

/// This always returns false, any cannot be equal
impl PartialEq for OtherParsedResult {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for OtherParsedResult {}
