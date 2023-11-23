use crate::tomasulo_sim::{Value,ValueInner};

use super::ROBID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// use for Fx
pub struct RFID(u8);
const REGFILE_COUNT:usize=16;

#[derive(Debug)]
pub struct FRegFile{
    inner: [FRFinner;REGFILE_COUNT]
}

#[derive(Debug)]
pub struct FRFinner{
    pub src: Option<ROBID>,
    pub value: Option<Value>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// use for Rx
pub struct RegID(u8);

impl RegID {
    pub fn new(id:u8)->RegID{
        RegID(id)
    }
}

impl RFID {
    pub fn new(id:u8)->RFID{
        RFID(id)
    }
}

impl FRegFile{
    pub fn new()->FRegFile{
        let inner: [FRFinner; REGFILE_COUNT] = Default::default();
        FRegFile { inner }
    }

    // get a FRF in RFID
    pub fn get(&self,id:&RFID)->&FRFinner{
        &self.inner[id.0 as usize]
    }

    pub fn get_mut(&mut self,id:&RFID)-> &mut FRFinner{
        &mut self.inner[id.0 as usize]
    }
}

impl FRFinner{
    pub fn new()->FRFinner{
        FRFinner { src:None, value: None }
    }
}

impl Default for FRFinner {
    fn default() -> Self {
        FRFinner { src: None, value: Some(Value::new(ValueInner::Float(1.0))) }
    }
}

impl std::fmt::Display for RFID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "F{:<1}",self.0)

    }
}

impl std::fmt::Display for RegID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{:<1}",self.0)
    }
}