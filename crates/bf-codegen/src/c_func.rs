use std::marker::PhantomData;

use bf_ir::ir::IR;

use crate::inst_translator::{CodeLine, InstTranslator};

pub struct CFunction {
    __: PhantomData<()>,
    
    pub name: String,
    pub lines: Vec<CodeLine>,
}

impl PartialEq for CFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for CFunction {}

impl PartialOrd for CFunction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for CFunction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl CFunction {
    pub fn signature(name: &str) -> String {
        format!("bf_bool_t {}(BFCalls calls, BFRuntimeEnv* env, BFRuntimeReport* report)", name)
    }
}

impl CFunction {
    pub fn new(name: &str, ir: IR, boundary_check: bool) -> Self {
        let mut lines = Vec::new();

        lines.append(&mut Self::prologue(name));

        let translator = InstTranslator::new(boundary_check);
        for inst in ir.instructions() {
            lines.append(&mut translator.translate_one(inst));
            // lines.push(CodeLine::new_empty_line());
        }

        lines.append(&mut Self::epilogue());

        Self {
            __: PhantomData,
            name: name.into(),
            lines,
        }
    }

    fn prologue(name: &str) -> Vec<CodeLine> {
        let mut lines = Vec::new();

        lines.push(CodeLine::new(
            0,
            format!("{} {{", Self::signature(name))
        ));

        lines.push(CodeLine::new(
            0,
            "// Preparation starts.".into()
        ));

        lines.push(CodeLine::new(
            4,
            "*report = (BFRuntimeReport){".into()
        ));
        
        lines.push(CodeLine::new(
            8,
            ".file_name = __FILE__,".into()
        ));
        
        lines.push(CodeLine::new(
            8,
            ".func_name = __func__,".into()
        ));

        lines.push(CodeLine::new(
            8,
            ".error_kind = NoBFError,".into()
        ));

        lines.push(CodeLine::new(
            4,
            "};".into()
        ));

        lines.push(CodeLine::new_empty_line());

        lines.push(CodeLine::new(
            4,
            "const bf_size_t TAPE_LEN = calls.tape_len(env);".into()
        ));

        lines.push(CodeLine::new(
            4,
            "bf_byte_t* tape = calls.get_tape(env);".into()
        ));
        
        lines.push(CodeLine::new(
            4,
            "bf_off_t pos = 0;".into()
        ));

        lines.push(CodeLine::new(
            0,
            "// Preparation ends.".into()
        ));
        
        lines.push(CodeLine::new_empty_line());

        lines.push(CodeLine::new(
            0,
            "// Code starts.".into()
        ));

        lines
    }

    fn epilogue() -> Vec<CodeLine> {
        let mut lines = Vec::new();

        lines.push(CodeLine::new(
            0,
            "// Code ends.".into()
        ));

        lines.push(CodeLine::new_empty_line());

        lines.push(CodeLine::new(
            4,
            "return BF_TRUE;".into()
        ));

        lines.push(CodeLine::new(
            0,
            "}".into()
        ));

        lines
    }
}
