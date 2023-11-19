use std::collections::BTreeMap;

use crate::tomasulo_sim::{Value, Instruction};

use super::Unit;

const INST_COUNT: usize = 8;

#[derive(Debug,Hash,PartialEq, PartialOrd, Ord, Eq,Clone,Copy)]
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
    pub dst: Option<Unit>,
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

    pub fn insert(&mut self,inst: &Instruction,entry: &usize){
        let robid=ROBID(*entry);
        if let Some(rob_entry) = self.inner.get_mut(&robid){
            rob_entry.entry=*entry;
            rob_entry.state=ROBState::Issue;
            rob_entry.dst.replace(inst.dest.clone());
        }
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