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
    
    
}