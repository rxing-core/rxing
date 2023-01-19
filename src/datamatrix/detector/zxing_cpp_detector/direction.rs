#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    LEFT = -1,
    RIGHT = 1,
}

impl From<Direction> for i32 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::LEFT => -1,
            Direction::RIGHT => 1,
        }
    }
}
