use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum OP {
    ATRIB,
    ADD,
    SUB,
    DIV,
    MULT,
    JMZ,
    JNZ,
    JGT,
    RET
}

impl fmt::Display for OP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OP::ATRIB => write!(f, "ATRIB"),
            OP::ADD => write!(f, "ADD"),
            OP::SUB => write!(f, "SUB"),
            OP::DIV => write!(f, "DIV"),
            OP::MULT => write!(f, "MULT"),
            OP::JMZ => write!(f, "JMZ"),
            OP::JNZ => write!(f, "JNZ"),
            OP::JGT => write!(f, "JGT"),
            OP::RET => write!(f, "RET"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterCodeRow {
    pub op: OP,
    pub end1: Option<String>,
    pub end2: Option<String>,
    pub end3: Option<String>
}