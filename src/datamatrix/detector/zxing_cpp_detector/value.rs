#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    INVALID = -1,
    WHITE = 0,
    BLACK = 1,
}
impl Value {
    pub fn isBlack(&self) -> bool {
        self == &Value::BLACK
    }
    pub fn isWhite(&self) -> bool {
        self == &Value::WHITE
    }
    pub fn isValid(&self) -> bool {
        self != &Value::INVALID
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => Value::BLACK,
            false => Value::WHITE,
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::INVALID => false,
            Value::WHITE => true,
            Value::BLACK => true,
        }
    }
}
