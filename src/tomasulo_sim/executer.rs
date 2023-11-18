use super::*;

#[derive(Debug)]
pub struct Executer{
    /// current cycle
    pub cycle:u8,

    pub rs: Reservation,
    pub rob: ReorderBuffer,
    pub freg: FRegFile,

    pub insts: Vec<Instruction>,
    pub commited_insts: Vec<Instruction>,
    pub insts_counts: usize,

    pub finished: bool,
}

impl Executer{
    pub fn new()->Executer{
        Executer { 
            cycle:0, 
            rs: Reservation::new(), 
            rob: ReorderBuffer::new(), 
            freg: FRegFile::new(), 
            insts: Vec::new(), 
            commited_insts: Vec::new(), 
            insts_counts: 0, 
            finished: false 
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Executer;

    #[test]
    fn exec_init(){
        let ex=Executer::new();
        println!("{:?}",ex);
    }
}