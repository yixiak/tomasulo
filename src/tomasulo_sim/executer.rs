use std::collections::VecDeque;
use console::style;

use super::*;

#[derive(Debug)]
pub struct Executor{
    /// current cycle
    pub cycle:u8,

    pub rs: Reservation,
    pub rob: ReorderBuffer,
    pub freg: FRegFile,

    pub insts: VecDeque<Instruction>,
    pub commited_insts: Vec<Instruction>,
    pub insts_counts: usize,
    // for issue and ROB entry 
    pub insts_issued: usize,

    pub finished: bool,
}

impl Executor{
    pub fn new()->Executor{
        Executor { 
            cycle:0, 
            rs: Reservation::new(), 
            rob: ReorderBuffer::new(), 
            freg: FRegFile::new(), 
            insts: VecDeque::new(), 
            commited_insts: Vec::new(), 
            insts_counts: 0, 
            insts_issued: 0,
            finished: false 
        }
    }

    pub fn add_inst(&mut self,inst_: &VecDeque<Instruction>){
        self.insts.extend(inst_.iter().cloned());
        self.insts_counts=self.insts.len();
    }

    pub fn issue(&mut self){
        // first, get the instruction form deque
        if let Some(inst) = self.insts.pop_front() {
            // get the free rs_entry
            if let Some(rs_entry_id) = self.rs.get_free(inst.op) {
                // put the inst into rs and rob
                // use insts_issued to index
                
                self.rs.insert(&inst, &mut self.freg,rs_entry_id,&self.rob,&self.cycle,&self.insts_issued);
                self.rob.insert(inst,&self.insts_issued);
                self.insts_issued += 1;
                return;
            }
            // there is no free rs
            self.insts.push_front(inst);
        };
    }

    pub fn calc(&mut self)->Vec<Instruction>{
        // the write back Instruction
        let wb_vec=self.rs.calc(&self.cycle);
        let comp=self.rob.calc(wb_vec,&mut self.rs,&self.cycle,&mut self.freg);
        comp
    }

    pub fn run(&mut self){
        while !self.finished{

            println!("{:=^60}", style("=").bold());
            
            self.cycle += 1;
            println!("{}{}",style(String::from("Cycle: ")).red().bold(),self.cycle);
            
            // issue one instruction pre cycle
            self.issue();
            
            // collect the finish instruction 
            let comp = self.calc();

            self.commited_insts.extend(comp.iter().cloned());

            self.finished = self.commited_insts.len()==self.insts_counts;

            println!("{}",style(String::from("Reorder Buffer:")).yellow().bold());
            println!("{}",self.rob);
            println!("{}",style(String::from("Reservation Station:")).blue().bold());
            println!("{}",self.rs);

            println!("{}",style(String::from("RegisterFile:")).black().bold());
            println!("{}",self.freg);
            
            println!("{:=^60}\n", style("=").bold());
            if self.cycle > 200 {
                panic!("Cycle limit exceeded.");
            }
        }
        println!("{}",style(String::from("Instruction:")).green().bold());
        println!("{:<16}{:<7} {:<7} {:<9} {:<7}",
            "",
            style(String::from("Issue")).bold(),
            style(String::from("ex_comp")).bold(),
            style(String::from("writeback")).bold(),
            style(String::from("commit")).bold(),
        );
        self.commited_insts.iter().for_each(|inst| println!("{}",inst));
    }
}

#[cfg(test)]
mod tests{
    use super::Executor;

    #[test]
    fn exec_init(){
        let ex=Executor::new();
        println!("{:?}",ex);
    }
}