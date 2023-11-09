use std::str::FromStr;


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