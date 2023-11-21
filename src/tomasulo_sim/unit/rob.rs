use std::collections::BTreeMap;

use crate::tomasulo_sim::{Value, Instruction, Reservation, RSState, RSType};

use super::{Unit, FRegFile};

const INST_COUNT: usize = 8;

#[derive(Debug,Hash,PartialEq, PartialOrd, Ord, Eq,Clone,Copy)]
pub struct ROBID(pub usize);

#[derive(Debug,PartialEq,Eq)]
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

    pub fn calc(&mut self,wv_vec:Vec<Instruction>,rs:&mut Reservation,cycle:&u8,freg:&mut FRegFile)->Vec<Instruction>{
        let mut comp = Vec::<Instruction>::new();

        for inst in wv_vec.iter(){
            if let Some(robid) = inst.robid{
               if let Some(rob_entry) = self.inner.get_mut(&robid){
                    rob_entry.state = ROBState::WriteBack;
                    let result = inst.apply();
                    rob_entry.value.replace(result.clone());
                    rob_entry.broadcast(rs);
               }
            }    
        }

        // commit
        for rob in self.inner.iter_mut(){
            if rob.1.state == ROBState::Commit {
                continue;
            }else if rob.1.state == ROBState::WriteBack{
                rob.1.commit(freg);
                let inst = rob.1.inst.as_mut().unwrap();
                inst.commit_cycle.replace(*cycle);
                comp.push(inst.clone());
            }else{
                break;
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

    pub fn broadcast(&self, rs:&mut Reservation){
        let robid=ROBID(self.entry.clone());
        let value = self.value.clone();
        rs.inner.iter_mut().for_each(|rs_entry|{
            if rs_entry.1.state == RSState::Waitting{
                if let Some(qj) = rs_entry.1.qj.clone(){
                    if qj==robid{
                        rs_entry.1.qj=None;
                        rs_entry.1.vj=value.clone();
                    }
                }
                if let Some(qk) = rs_entry.1.qk.clone(){
                    if qk==robid{
                        rs_entry.1.qk=None;
                        rs_entry.1.vk=value.clone();
                    }
                }

                // check the instruction's src 
                match rs_entry.1.op {
                    RSType::LD | RSType::SD => {
                        if rs_entry.1.vk.is_some() {
                            rs_entry.1.state=RSState::Ready;
                        }
                    }
                    _ => {
                        if rs_entry.1.vj.is_some() && rs_entry.1.vk.is_some(){
                            rs_entry.1.state=RSState::Ready;
                        }
                    }
                }
            }
        }
    )
    }

    pub fn commit(&self,freg:&mut FRegFile){
        let value = self.value.clone();
        let dest = self.inst.as_ref().unwrap().dest.clone();
        match dest {
            Unit::RF(id)=>{
                let reg = freg.get_mut(&id);
                if reg.src.unwrap().clone()==ROBID(self.entry.clone()){
                    reg.value = Some(value.unwrap());
                }
            },
            _=>{}
        }
    }
}