use crate::tomasulo_sim::value::Value;

use super::ROBID;


const REGFILE_COUNT:usize=16;

pub struct RegFile{
    inner: [PFinner;REGFILE_COUNT]
}

pub struct PFinner{
    pub src: Option<ROBID>,
    pub value: Option<Value>
}