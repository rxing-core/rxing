#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Invalid = -1,
    White = 0,
    Black = 1,
}
impl Value {
    pub fn isBlack(&self) -> bool {
        self == &Value::Black
    }
    pub fn isWhite(&self) -> bool {
        self == &Value::White
    }
    pub fn isValid(&self) -> bool {
        self != &Value::Invalid
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => Value::Black,
            false => Value::White,
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Invalid => false,
            Value::White => true,
            Value::Black => true,
        }
    }
}
