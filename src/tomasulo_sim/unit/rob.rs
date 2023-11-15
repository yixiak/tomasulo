use std::collections::BTreeMap;

const INST_COUNT: usize = 8;

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
    pub value: i8,
    //pub dst: register
    pub state: ROBState,
} 

pub struct ReorderBuffer{
    pub inner: BTreeMap<ROBID, ROBInner>,
}