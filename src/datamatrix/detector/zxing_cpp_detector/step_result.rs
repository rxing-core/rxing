#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StepResult {
    Found,
    OpenEnd,
    ClosedEnd,
}
