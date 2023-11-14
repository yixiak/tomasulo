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
    Free
}

pub struct RSinner{
    pub state: RSState,
    pub vj: u8,
    pub vk: u8,
    //pub qj: ROB
    //pub qk: ROB
    //pub dest: register
}

pub struct Reservation{
    pub inner: BTreeMap<RSId,RSinner>
}