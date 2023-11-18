use std::collections::BTreeMap;

use crate::tomasulo_sim::Value;

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

#[derive(Debug)]
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

    pub op: RSType,
    pub state: RSState,
    pub vj: Option<Value>,
    pub vk: Option<Value>,
    pub qj: Option<Value>,
    pub qk: Option<Value>,
    pub dest: Option<Value>,

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
}

impl RSinner {
    pub fn new(optype:RSType,id:usize)->RSinner {
        RSinner { 
            id:RSId(id, optype),
            op: optype, 
            state: RSState::Free, 
            vj: None, 
            vk: None, 
            qj: None, 
            qk: None, 
            dest: None, 
            issue_cycle: None, 
            execute_begin_cycle: None, 
            execute_cycle: None, 
            write_back_cycle: None 
        }
    }
}