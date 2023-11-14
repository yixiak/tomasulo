use super::rs::RSType;

pub struct FU{
    pub op:RSType,
    pub remaining_cycles: Option<u8>
}