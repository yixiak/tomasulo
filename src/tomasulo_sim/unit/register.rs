use crate::tomasulo_sim::Value;

use super::ROBID;

#[derive(Debug)]
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

#[derive(Debug)]
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
}

impl FRFinner{
    pub fn new()->FRFinner{
        FRFinner { src:None, value: None }
    }
}

impl Default for FRFinner {
    fn default() -> Self {
        FRFinner { src: None, value: None }
    }
}