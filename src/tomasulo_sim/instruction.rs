use std::str::FromStr;

use super::{Value, Unit};
#[derive(Debug)]
pub enum Type {
    LD,
    SD,
    ADDD,
    SUBD,
    MULTD,
    DIVD,
}

#[derive(Debug)]
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

// get a Instruction form str
impl FromStr for Instruction{
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let mut parts=s.split_whitespace();
        let op= parts.next().expect("no op").parse()?;
        let dest = parts.next().expect("no dest").parse()?;

        // src maybe unit or imm+
        let mut src1 = None;
        let mut src2 = None;
        let next = parts.next().expect("no src1");
        src1.replace(Value::new( match next.parse::<Unit>() {
            Ok(u)=>u.into(),
            Err(_)=>{
                // remove the +
                let trimmed = next.strip_suffix('+').unwrap_or(next);
                trimmed.parse::<i64>().map_err(|_| ())?.into()
            }
        }));

        let next=parts.next().expect("no src2");
        src2.replace(Value::new( match next.parse::<Unit>() {
            Ok(u)=>u.into(),
            Err(_)=>{
                // remove the +
                let trimmed = next.strip_suffix('+').unwrap_or(next);
                trimmed.parse::<i64>()
                        .map_err(|_| ())?
                        .into()
            }
        }));
        return Ok(Self { 
            op, 
            src1,
            src2,
            dest,
            issue_cycle: None, 
            execute_begin_cycle: None, 
            execute_end_cycle: None, 
            write_back_cycle: None, 
            commit_cycle: None })
    }
}

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

#[cfg(test)]
mod tests {
    use crate::tomasulo_sim::Instruction;

    #[test]
    fn src_to_inst(){
        let input1:[&str;6]=[
        "LD F6 34+ R2",
        "LD F2 45+ R3",
        "MULTD F0 F2 F4",
        "SUBD F8 F6 F2",
        "DIVD F10 F0 F6",
        "ADDD F6 F8 F2"
    ];
    let input2:[&str;8]=[
        "LD F2 0 R2",
        "LD F4 0 R3",
        "DIVD F0 F4 F2",
        "MULTD F6 F0 F2",
        "ADDD F0 F4 F2",
        "SD F6 0 R3",
        "MULTD F6 F0 F2",
        "SD F6 0 R1"
    ];
        for &inst in input1.iter(){
            if let Ok(i) = inst.trim().parse::<Instruction>(){
                                    println!("{:?}",i);
                                }
        };
        for &inst in input2.iter(){
            if let Ok(i) = inst.trim().parse::<Instruction>(){
                                    println!("{:?}",i);
                                }
        };
    }
}
