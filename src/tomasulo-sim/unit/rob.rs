use std::collections::BTreeMap;

const INST_COUNT: usize = 8;

pub enum ROB_State{
    Issue,
    Execute,
    WriteBack,
    Commit,
    Waitting
}

pub struct ROB_inner{
    pub entry: u8,
    //pub type:
    pub value: i8,
    //pub dst: register
    pub state: ROB_State,
} 

pub struct RecordBuffer{
    inner: BTreeMap<u8, ROB_inner>,
}