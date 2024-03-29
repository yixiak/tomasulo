use std::str::FromStr;

use console::style;

use super::{Value, Unit, ROBID};

const LD_CYCLE:u8 = 2;
const ADD_CYCLE:u8 = 2;
const MULT_CYCLE:u8 = 10;
const DIVD_CYCLE:u8 = 20;

#[derive(Debug,Clone,Copy)]
pub enum Type {
    LD,
    SD,
    ADDD,
    SUBD,
    MULTD,
    DIVD,
}

#[derive(Debug,Clone)]
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

    pub required_cycle: u8,    
    pub robid: Option<ROBID>,
    result: Option<Value>
    
}


// get a Instruction form str
impl FromStr for Instruction{
    type Err=();

    #[warn(unused_assignments)]
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

        let required_cycle:u8;

        match op {
            Type::LD | Type::SD => required_cycle=LD_CYCLE,
            Type::ADDD | Type::SUBD => required_cycle=ADD_CYCLE,
            Type::MULTD => required_cycle=MULT_CYCLE,
            Type::DIVD => required_cycle=DIVD_CYCLE,
        }

        return Ok(Self { 
            op, 
            src1,
            src2,
            dest,
            issue_cycle: None, 
            execute_begin_cycle: None, 
            execute_end_cycle: None, 
            write_back_cycle: None, 
            commit_cycle: None,
            required_cycle,
            robid: None, 
            result: None,
        })
    }
}

impl FromStr for Type{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADDD"=>Ok(Type::ADDD),
            "SUBD"=>Ok(Type::SUBD),
            "MULTD"=>Ok(Type::MULTD),
            "DIVD"=>Ok(Type::DIVD),
            "LD"=>Ok(Type::LD),
            "SD"=>Ok(Type::SD),
            _=>Err(()),
        }
    }
}

impl Instruction{
    
    pub fn result(&self) -> Option<Value>{
        self.result.clone()
    }
    pub fn set_result(&mut self, value:Value){
        self.result.replace(value);
    }
}


impl Type {
    pub fn op_str(&self) -> &'static str {
        match self {
            Type::ADDD => "+",
            Type::SUBD => "-",
            Type::MULTD => "*",
            Type::DIVD => "/",
            _ => "",
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Type::ADDD => style(format!("{self:?}")).green(),
            Type::SUBD => style(format!("{self:?}")).red(),
            Type::MULTD => style(format!("{self:?}")).yellow(),
            Type::DIVD => style(format!("{self:?}")).blue(),
            Type::LD => style(format!("{self:?}")).cyan(),
            Type::SD => style(format!("{self:?}")).magenta(),
        };
        write!(f, "{s:<5}")
    }
}

impl std::fmt::Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let issue = self.issue_cycle.unwrap().clone();
        let ex_end = self.execute_end_cycle.unwrap().clone();
        let write_back = self.write_back_cycle.unwrap().clone();
        let commit = self.commit_cycle.unwrap().clone();
        // let mut ex_begin :u8 = 0;
        // if self.execute_begin_cycle.is_none(){
        //     match self.op{
        //         Type::LD | Type::SD => {ex_begin = ex_end-LD_CYCLE+1},
        //         Type::ADDD | Type::SUBD => {ex_begin = ex_end-ADD_CYCLE+1},
        //         Type::MULTD => {ex_begin = ex_end-MULT_CYCLE+1},
        //         Type::DIVD => {ex_begin = ex_end-DIVD_CYCLE+1}

        //     }
        // }else{
        //     ex_begin = self.execute_begin_cycle.unwrap().clone();
        // }
        
        //let ex_begin = self.execute_begin_cycle.unwrap().clone();
        write!(f,
            "{} {:<3} {:<3} {:<3}: {:<7} {:<7} {:<9} {:<7};",
            self.op,self.dest,self.src1.as_ref().unwrap().clone(),self.src2.as_ref().unwrap().clone(),
            issue,ex_end,write_back,commit
        )
    }
}
