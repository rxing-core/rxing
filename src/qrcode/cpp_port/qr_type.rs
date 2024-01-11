#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Model1,
    Model2,
    Micro,
}

impl Type {
    pub const fn const_eq(a: Type, b: Type) -> bool {
        let (a, b) = (a as u8, b as u8);

        a == b
    }
}
