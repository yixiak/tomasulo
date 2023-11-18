use super::*;

pub struct Executer{
    /// current cycle
    pub cycle:u8,
    pub rs:Reservation,
    pub rob:ReorderBuffer,

}