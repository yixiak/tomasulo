use std::collections::BTreeMap;

use crate::tomasulo_sim::Value;

const INST_COUNT: usize = 8;

#[derive(Debug,Hash,PartialEq, PartialOrd, Ord, Eq)]
pub struct ROBID(usize);

#[derive(Debug)]
pub enum ROBState{
    Free,
    Issue,
    Execute,
    WriteBack,
    Commit,
    Waitting
}

#[derive(Debug)]
pub struct ROBInner{
    pub entry: usize,
    //pub type:
    pub value: Option<Value>,
    pub dst: Option<Value>,
    pub state: ROBState,
} 

#[derive(Debug)]
pub struct ReorderBuffer{
    pub inner: BTreeMap<ROBID, ROBInner>,
}


impl ReorderBuffer{
    pub fn new()->ReorderBuffer{
        let mut inner=BTreeMap::new();

        for index in 0..INST_COUNT{
            inner.insert(
                ROBID(index), 
                ROBInner::new(index)
            );
        }

        ReorderBuffer { inner }
    }
}

impl ROBInner{
    pub fn new(id:usize)->ROBInner{
        ROBInner{
            entry: id,
            value: None,
            dst: None,
            state: ROBState::Free,
        }
    }
}