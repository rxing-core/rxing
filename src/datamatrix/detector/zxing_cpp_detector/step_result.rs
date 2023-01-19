#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StepResult {
    FOUND,
    OPEN_END,
    CLOSED_END,
}
