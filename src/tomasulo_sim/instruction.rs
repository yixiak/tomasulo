use std::str::FromStr;

use super::Value;
pub enum Type {
    LD,
    SD,
    ADDD,
    SUBD,
    MULTD,
    DIVD,
}

pub struct Instruction{
    pub op:Type,

    pub src1: Option<Value>,
    pub src2: Option<Value>,
    pub dest: Option<Value>,

    // when the instruction issued,
    // it may waitting for some value to execute
    // and has no these cycle_values,
    // so we use Option<>.
    pub issue_cycle: Option<u8>,
    pub execute_begin_cycle: Option<u8>,
    pub execute_end_cycle: Option<u8>,
    pub write_back_cycle: Option<u8>,
    pub commit_cycle: Option<u8>,
    
    
}

// get a Instruction for str
impl FromStr for Instruction{
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let parts=s.split_whitespace();

        return Ok(Self { op: (), issue_cycle: (), execute_begin_cycle: (), execute_end_cycle: (), write_back_cycle: (), commit_cycle: () })
    }
}
