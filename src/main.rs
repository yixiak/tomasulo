use std::collections::VecDeque;

use tomasulo_sim::{Executor, Instruction};

pub mod tomasulo_sim;
fn main() {
    // test instruction
    let input1:[&str;6]=[
        "LD F6 34+ R2",
        "LD F2 45+ R3",
        "MULTD F0 F2 F4",
        "SUBD F8 F6 F2",
        "DIVD F10 F0 F6",
        "ADDD F6 F8 F2"
    ];

    let input2:[&str;8]=[
        "LD F2 0 R2",
        "LD F4 0 R3",
        "DIVD F0 F4 F2",
        "MULTD F6 F0 F2",
        "ADDD F0 F4 F2",
        "SD F6 0 R3",
        "MULTD F6 F0 F2",
        "SD F6 0 R1"
    ];

    let mut ex1=Executor::new();
    let mut ex2=Executor::new();
    let i1=input1
                                .iter()
                                .map(|&s| 
                                    match s.parse::<Instruction>(){
                                        Ok(inst)=>inst,
                                        Err(_)=>{panic!("Instruction parse error!");
                                        }
                                    }
                                )
                                .collect();


    let i2:VecDeque<Instruction>=input2
                                .iter()
                                .map(|&s| 
                                    match s.parse::<Instruction>(){
                                        Ok(inst)=>inst,
                                        Err(_)=>{panic!("Instruction parse error!");
                                        }
                                    }
                                )
                                .collect();
    ex1.add_inst(&i1);
    ex1.run();
    ex2.add_inst(&i2);
    ex2.run();
    println!("End");


    
}
