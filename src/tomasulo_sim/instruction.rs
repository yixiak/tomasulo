use std::str::FromStr;

use super::{Value, Unit};
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
    pub dest: Unit,

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

// // get a Instruction form str
// impl FromStr for Instruction{
//     type Err=();

//     fn from_str(s: &str) -> Result<Self, Self::Err>{
//         let mut parts=s.split_whitespace();
//         let op= parts.next().expect("no op").parse()?;
//         let dest = parts.next().parse()?;
//         let src1 = parts.next().parse()?;
//         let src2 = parts.next().parse()?;
        
//         return Ok(Self { 
//             op, 
//             src1,
//             src2,
//             dest,
//             issue_cycle: None, 
//             execute_begin_cycle: None, 
//             execute_end_cycle: None, 
//             write_back_cycle: None, 
//             commit_cycle: None })
//     }
// }

impl FromStr for Type{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADDD"=>Ok(Type::ADDD),
            "SUB"=>Ok(Type::SUBD),
            "MULTD"=>Ok(Type::MULTD),
            "DIVD"=>Ok(Type::DIVD),
            "LD"=>Ok(Type::LD),
            "SD"=>Ok(Type::SD),
            _=>Err(()),
        }
    }
}
