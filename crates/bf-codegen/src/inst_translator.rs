use bf_ir::inst::{Inst, Label};

use crate::code_file::CodeLine;

pub struct InstTranslator {
    boundary_check: bool,
}

impl InstTranslator {
    pub fn new(boundary_check: bool) -> Self {
        Self {
            boundary_check,
        }
    }
}

impl InstTranslator {
    pub fn translate(self, insts: &[Inst]) -> Vec<CodeLine> {
        let mut lines = Vec::new();

        for inst in insts {
            let mut sub_lines = self.translate_one(inst);
            lines.append(&mut sub_lines);
        }

        lines
    }
    
    pub fn translate_one(&self, inst: &Inst) -> Vec<CodeLine> {
        let mut lines = Vec::new();

        match inst {
            Inst::Add(n) => {
                lines.push(CodeLine::new(
                    4,
                    format!("tape[pos] += {};", *n)
                ));
            }

            Inst::Move(n) => {
                lines.push(CodeLine::new(
                    4,
                    "{".into()
                ));

                lines.push(CodeLine::new(
                    8,
                    format!("size_t target_pos = pos + {};", *n)
                ));
                
                if self.boundary_check {
                    lines.push(CodeLine::new(
                        8,
                        "if (target_pos >= TAPE_LEN) {".into()
                    ));

                    lines.push(CodeLine::new(
                        12,
                        "fprintf(stderr, \"Access tape[%zu]: Out of tape(len: %zu) boundary.\", target_pos, TAPE_LEN);".into()
                    ));

                    lines.push(CodeLine::new(
                        12,
                        "exit(1);".into()
                    ));

                    lines.push(CodeLine::new(
                        8,
                        "}".into()
                    ));
                }

                lines.push(CodeLine::new(
                    8,
                    "pos = target_pos;".into()
                ));

                lines.push(CodeLine::new(
                    4,
                    "}".into()
                ));
            }

            Inst::Input => {
                lines.push(CodeLine::new(
                    4,
                    "tape[pos] = getchar();".into(),
                ));
            }

            Inst::Output => {
                lines.push(CodeLine::new(
                    4,
                    "putchar(tape[pos]);".into(),
                ));
            }

            Inst::Label(Label(id)) => {
                lines.push(CodeLine::new(
                    0,
                    format!("L{}:", *id)
                ));
            }

            Inst::Jump(Label(id)) => {
                lines.push(CodeLine::new(
                    4,
                    format!("goto L{};", *id)
                ));
            }

            Inst::JumpIfZero(Label(id)) => {
                lines.push(CodeLine::new(
                    4,
                    "if (0 == tape[pos])".into()
                ));

                lines.push(CodeLine::new(
                    8,
                    format!("goto L{};", *id)
                ));
            }
        }
        
        lines
    }
}
