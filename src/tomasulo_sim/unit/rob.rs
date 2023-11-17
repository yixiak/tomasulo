use std::collections::BTreeMap;

use crate::tomasulo_sim::Value;

const INST_COUNT: usize = 8;

#[derive(Debug)]
pub struct ROBID(u8);

pub enum ROBState{
    Issue,
    Execute,
    WriteBack,
    Commit,
    Waitting
}

pub struct ROBInner{
    pub entry: u8,
    //pub type:
    pub value: Option<Value>,
    //pub dst: register
    pub state: ROBState,
} 

pub struct ReorderBuffer{
    pub inner: BTreeMap<ROBID, ROBInner>,
}