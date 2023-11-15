use std::collections::BTreeMap;

pub enum RSType{
    LD,
    SD,
    ADD,
    MULT,
}

pub struct RSId(u8,RSType);

pub enum RSState{
    Busy,
    Ready,
    Executing,
    Waitting,
    Free
}

pub struct RSinner{
    pub op: RSType,
    pub state: RSState,
    //pub vj: u8,
    //pub vk: u8,
    //pub qj: ROB
    //pub qk: ROB
    //pub dest: register

    // when the instruction issued,
    // it may waitting for some value to execute
    // and has no these cycle_values,
    // so we use Option<>.
    pub issue_cycle: Option<u8>,
    pub execute_begin_cycle: Option<u8>,
    pub execute_cycle: Option<u8>,
    pub write_back_cycle: Option<u8>,
}

pub struct Reservation{
    pub inner: BTreeMap<RSId,RSinner>
}