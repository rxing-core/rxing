#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredAppendInfo {
    pub index: i32, // -1;
    pub count: i32, // = -1;
    pub id: String,
}

impl Default for StructuredAppendInfo {
    fn default() -> Self {
        Self {
            index: -1,
            count: -1,
            id: Default::default(),
        }
    }
}
