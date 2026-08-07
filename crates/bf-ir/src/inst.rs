#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label(pub usize);

#[derive(Debug, Clone)]
pub enum Inst {
    Add(i8),
    Move(i32),
    Input,
    Output,
    Jump(Label),
    JumpIfZero(Label),
    Label(Label),
}
