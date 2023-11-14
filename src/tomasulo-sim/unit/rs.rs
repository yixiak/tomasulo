use std::collections::BTreeMap;

pub enum RS_Type{
    LD,
    SD,
    ADD,
    MULT,
}

pub struct RSId(u8,RS_Type);

pub enum RS_State{
    Busy,
    Free
}

pub struct RS_inner{
    pub state: RS_State,
    pub vj: u8,
    pub vk: u8,
    //pub qj: ROB
    //pub qk: ROB
    //pub dest: register
}

pub struct Reservation{
    pub inner: BTreeMap<RSId,RS_inner>
}