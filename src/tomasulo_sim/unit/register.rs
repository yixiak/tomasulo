use crate::tomasulo_sim::Value;

use super::ROBID;

#[derive(Debug)]
// use for Fx
pub struct RFID(u8);
const REGFILE_COUNT:usize=16;

pub struct RegFile{
    inner: [RFinner;REGFILE_COUNT]
}

pub struct RFinner{
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