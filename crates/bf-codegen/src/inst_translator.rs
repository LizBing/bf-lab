use bf_ir::inst::{Inst, Label};

pub struct CodeLine {
    indents: usize,
    content: String,
}

impl CodeLine {
    pub fn new(indents: usize, content: String) -> Self {
        Self {
            indents,
            content,
        }
    }

    pub fn new_empty_line() -> Self {
        Self::new(0, "".into())
    }
}

impl CodeLine {
    pub fn as_string(&self) -> String {
        let spaces: String = std::iter::repeat(' ').take(self.indents).collect();
        format!("{}{}", spaces, self.content)
    }
}


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
                    format!("bf_off_t new_pos = pos + {};", *n)
                ));

                if self.boundary_check {
                    let err_kind_str = if *n >= 0 {
                        "BFErrorOverflow"
                    } else {
                        "BFErrorUnderflow"
                    };

                    lines.push(CodeLine::new(
                        8,
                        "if (new_pos >= TAPE_LEN) {".into()
                    ));

                    lines.push(CodeLine::new(
                        12,
                        format!("report->error_kind = {};", err_kind_str)
                    ));

                    lines.push(CodeLine::new(
                        12,
                        "return BF_FALSE;".into()
                    ));

                    lines.push(CodeLine::new(
                        8,
                        "}".into()
                    ));
                }

                lines.push(CodeLine::new(
                    8,
                    "pos = new_pos;".into()
                ));

                lines.push(CodeLine::new(
                    4,
                    "}".into()
                ));
            }

            Inst::Input => {
                lines.push(CodeLine::new(
                    4,
                    "if (!calls.getchar(env, tape + pos)) {".into()
                ));

                lines.push(CodeLine::new(
                    8,
                    "report->error_kind = BFErrorInStream;".into()
                ));

                lines.push(CodeLine::new(
                    8,
                    "return BF_FALSE;".into()
                ));

                lines.push(CodeLine::new(
                    4,
                    "}".into(),
                ));
            }

            Inst::Output => {
                lines.push(CodeLine::new(
                    4,
                    "if (!calls.putchar(env, tape[pos])) {".into()
                ));

                lines.push(CodeLine::new(
                    8,
                    "report->error_kind = BFErrorOutStream;".into()
                ));

                lines.push(CodeLine::new(
                    8,
                    "return BF_FALSE;".into()
                ));

                lines.push(CodeLine::new(
                    4,
                    "}".into(),
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
                    format!("if (0 == tape[pos]) goto L{};", *id)
                ));
            }
        }

        lines
    }
}
