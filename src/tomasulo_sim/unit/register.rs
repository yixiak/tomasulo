use crate::tomasulo_sim::Value;

use super::ROBID;

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

// use for Rx
pub struct RegID(u8);