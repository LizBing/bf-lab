use crate::inst::Inst;

pub struct PeepholeOptimizer<'a> {
    target: &'a [Inst],
    pos: usize,
}

impl<'a> PeepholeOptimizer<'a> {
    pub fn new(target: &'a [Inst]) -> Self {
        Self { target, pos: 0 }
    }
}

impl PeepholeOptimizer<'_> {
    fn peek(&self) -> Option<&Inst> {
        self.target.get(self.pos)
    }

    fn step(&mut self) {
        debug_assert!(self.peek().is_some());
        self.pos += 1;
    }
}

impl PeepholeOptimizer<'_> {
    fn compact_adds(&mut self) -> Option<Inst> {
        let mut value: i8 = 0;

        while let Some(Inst::Add(n)) = self.peek() {
            value = value.wrapping_add(*n);
            self.step();
        }

        if value != 0 {
            Some(Inst::Add(value))
        } else {
            None
        }
    }

    fn compact_moves(&mut self) -> Option<Inst> {
        let mut value = 0;

        while let Some(Inst::Move(n)) = self.peek() {
            value += *n;
            self.step();
        }

        if value != 0 {
            Some(Inst::Move(value))
        } else {
            None
        }
    }

    fn clean_jump(&mut self) -> Option<Inst> {
        let jump = self.peek().unwrap().clone();
        let jump_lable = match jump {
            Inst::Jump(l) => l,
            Inst::JumpIfZero(l) => l,
            _ => unreachable!(),
        };

        self.step();

        let label = match self.peek() {
            Some(Inst::Label(l)) => l,
            _ => return Some(jump),
        };

        if jump_lable.eq(label) {
            None
        } else {
            Some(jump)
        }
    }
}

impl PeepholeOptimizer<'_> {
    pub fn optimize(mut self) -> Vec<Inst> {
        let mut res = Vec::new();

        loop {
            match self.peek() {
                None => break,

                Some(Inst::Add(_)) => {
                    if let Some(inst) = self.compact_adds() {
                        res.push(inst);
                    }
                }

                Some(Inst::Move(_)) => {
                    if let Some(inst) = self.compact_moves() {
                        res.push(inst);
                    }
                }

                Some(Inst::Input) => {
                    res.push(Inst::Input);
                    self.step();
                }

                Some(Inst::Output) => {
                    res.push(Inst::Output);
                    self.step();
                }

                Some(Inst::Jump(_) | Inst::JumpIfZero(_)) => {
                    if let Some(inst) = self.clean_jump() {
                        res.push(inst);
                    }
                }

                Some(Inst::Label(l)) => {
                    res.push(Inst::Label(*l));
                    self.step();
                }
            }
        }

        res
    }
}
