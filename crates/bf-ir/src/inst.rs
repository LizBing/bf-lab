pub enum Inst {
    Add(i8),
    Move(i32),
    Input,
    Output,
    Jump(usize),
    JumpIfZero(usize)
}
