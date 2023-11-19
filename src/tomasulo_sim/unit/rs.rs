use std::collections::BTreeMap;

use crate::tomasulo_sim::{Value, Type, Instruction, ValueInner};

use super::{FRegFile, ReorderBuffer, Unit, ROBID};

const LD_RS_COUNT:usize = 3;
const SD_RS_COUNT:usize = 3;
const ADD_RS_COUNT:usize = 2;
const MULT_RS_COUNT:usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RSType{
    LD,
    SD,
    ADD,
    MULT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RSId(usize,RSType);

#[derive(Debug,PartialEq,Eq)]
pub enum RSState{
    Busy,
    Ready,
    Executing,
    Waitting,
    Free
}

#[derive(Debug)]
pub struct RSinner{
    pub id: RSId,
    pub inst: Option<Instruction>,

    pub op: RSType,
    pub state: RSState,
    pub vj: Option<Value>,
    pub vk: Option<Value>,
    pub qj: Option<ROBID>,
    pub qk: Option<ROBID>,
    pub result: Option<Value>,
    // I can omit this attribute, but not the computer.
    pub addr: Option<Value>,

    // when the instruction issued,
    // it may waitting for some value to execute
    // and has no these cycle_values,
    // so we use Option<>.
    pub issue_cycle: Option<u8>,
    pub execute_begin_cycle: Option<u8>,
    pub execute_cycle: Option<u8>,
    pub write_back_cycle: Option<u8>,
}

#[derive(Debug)]
pub struct Reservation{
    pub inner: BTreeMap<RSId,RSinner>,
}

impl Reservation {
    pub fn new()->Reservation{
        let mut inner=BTreeMap::new();
        // LD 
        for index in 0..LD_RS_COUNT{
            inner.insert(
                RSId(index, RSType::LD),
                RSinner::new(RSType::LD,index));
        }
        // SD
        for index in 0..SD_RS_COUNT{
            inner.insert(
                RSId(index, RSType::SD),
                RSinner::new(RSType::SD,index));
        }
        
        for index in 0..ADD_RS_COUNT{
            inner.insert(
                RSId(index, RSType::ADD),
                RSinner::new(RSType::ADD,index));
        }
        for index in 0..MULT_RS_COUNT{
            inner.insert(
                RSId(index, RSType::MULT),
                RSinner::new(RSType::MULT,index));
        }

        Reservation { inner }
    }

    pub fn get_free(&self, op:Type)->Option<RSId>{
        let ty:RSType=op.clone().into();

        for inner in self.inner.values() {
            if inner.op == ty && inner.state == RSState::Free{
                return Some(inner.id);
            }
        }
        None
    }

    pub fn insert(&mut self,inst:Instruction,freg:&FRegFile,id:RSId,cycle:&u8){
        if let Some(rs_inner_entry) = self.inner.get_mut(&id){
            rs_inner_entry.modify(&inst,freg,id.0,cycle);
            rs_inner_entry.inst.replace(inst);
            rs_inner_entry.state=RSState::Busy;

        }
    }
}

impl RSinner {
    pub fn new(optype:RSType,id:usize)->RSinner {
        RSinner { 
            id:RSId(id, optype),
            inst:None,
            op: optype, 
            state: RSState::Free, 
            vj: None, 
            vk: None, 
            qj: None, 
            qk: None, 
            addr: None,
            result: None,
            issue_cycle: None, 
            execute_begin_cycle: None, 
            execute_cycle: None, 
            write_back_cycle: None 
        }
    }

    // modify rs from inst
    pub fn modify(&mut self,inst:&Instruction,freg:&FRegFile,id:usize,cycle:&u8){
        // use clone to aviod Transferting of ownership
        self.id=RSId(id,inst.op.clone().into());
        self.op=inst.op.clone().into();
        self.state=RSState::Busy;
        match inst.op {
            Type::LD => {
                // because the LD instructions on the top,
                // we assume that the value is always ready
                self.addr.replace(inst.src1.as_ref().unwrap().clone());
                self.vk.replace(inst.src2.as_ref().unwrap().clone());
                self.issue_cycle.replace(*cycle);
                
            },
            Type::SD => {
                self.addr.replace(inst.src1.as_ref().unwrap().clone());
                self.vj.replace(inst.src2.as_ref().unwrap().clone());
                self.issue_cycle.replace(*cycle);

                // the dest 
                if let Unit::RF(fregid)= inst.dest {
                    let reg = freg.get(&fregid);

                    match &reg.value{
                        // there is not a value stored in the reg
                        Some(value)=>{
                            self.vk.replace(value.clone());
                        }
                        None=>{
                            self.qk = reg.src;
                        }
                    }
                }

            },
            _=>{
                if let Some(src1)=inst.src1.clone(){
                    match *src1{
                        ValueInner::Unit(Unit::RF(rfid))=>{
                            let reg = freg.get(&rfid);
                            match &reg.value {
                                Some(value)=>{
                                    self.vj.replace(value.clone());
                                }
                                None=>{
                                    self.qj=reg.src;
                                }
                            }
                        }
                        _=>{panic!("src1 error")}
                    }
                }

                if let Some(src2)=inst.src2.clone(){
                    match *src2{
                        ValueInner::Unit(Unit::RF(rfid))=>{
                            let reg = freg.get(&rfid);
                            match &reg.value {
                                Some(value)=>{
                                    self.vk.replace(value.clone());
                                }
                                None=>{
                                    self.qk=reg.src;
                                }
                            }
                        }
                        _=>{panic!("src2 error")}
                    }
                }
                self.issue_cycle.replace(*cycle);
            }
        }

    }

}

impl From<Type> for RSType{
    fn from(value: Type) -> Self {
        match value {
            Type::LD => RSType::LD,
            Type::SD => RSType::LD,
            Type::ADDD => RSType::ADD,
            Type::SUBD => RSType::ADD,
            Type::MULTD => RSType::MULT,
            Type::DIVD => RSType::MULT,
        }
    }
}