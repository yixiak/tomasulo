use std::collections::BTreeMap;

use crate::tomasulo_sim::{Value, Instruction, Reservation};

use super::Unit;

const INST_COUNT: usize = 8;

#[derive(Debug,Hash,PartialEq, PartialOrd, Ord, Eq,Clone,Copy)]
pub struct ROBID(pub usize);

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
    pub inst: Option<Instruction>,
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

    pub fn insert(&mut self,inst: Instruction,entry: &usize){
        let robid=ROBID(*entry);
        if let Some(rob_entry) = self.inner.get_mut(&robid){
            rob_entry.entry=*entry;
            rob_entry.state=ROBState::Issue;
            rob_entry.dst.replace(inst.dest.clone());
            rob_entry.inst.replace(inst);
        }
    }

    pub fn calc(&mut self,wv_vec:Vec<Instruction>,rs:&mut Reservation)->Vec<Instruction>{
        let mut comp = Vec::<Instruction>::new();

        for inst in wv_vec.iter(){
            if let Some(robid) = inst.robid{
               if let Some(rob_entry) = self.inner.get_mut(&robid){
                    rob_entry.state = ROBState::WriteBack;
                    rob_entry.value.replace(inst.apply());
               }

                
            }
            
        }

        comp


    }
}

impl ROBInner{
    pub fn new(id:usize)->ROBInner{
        ROBInner{
            entry: id,
            inst: None,
            value: None,
            dst: None,
            state: ROBState::Free,
        }
    }

}