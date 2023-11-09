use super::instruction::Type;

pub struct FU{
    pub op:Type,

    pub remaining_cycles: Option<u8>
}