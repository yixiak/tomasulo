use std::{collections::BTreeMap, string, fmt::format};

use console::style;

use crate::tomasulo_sim::{Value, Type, Instruction, ValueInner, apply_op};

use super::{FRegFile, Unit, ROBID, ReorderBuffer};

const LD_RS_COUNT:usize = 3;
const SD_RS_COUNT:usize = 3;
const ADD_RS_COUNT:usize = 2;
const MULT_RS_COUNT:usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RSType{
    LD,
    SD,
    ADD,
    MULT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RSId(usize,RSType);

#[derive(Debug,PartialEq,Eq)]
pub enum RSState{
    Begin,
    Finished,
    Ready,
    Executing,
    Waitting,
    Free
}

#[derive(Debug)]
pub struct RSinner{
    pub id: RSId,
    pub inst: Option<Instruction>,

    pub op: RSType,
    pub state: RSState,
    pub vj: Option<Value>,
    pub vk: Option<Value>,
    pub qj: Option<ROBID>,
    pub qk: Option<ROBID>,
    pub result: Option<Value>,
    // I can omit this attribute, but not the computer.
    pub addr: Option<Value>,

    // when the instruction issued,
    // it may waitting for some value to execute
    // and has no these cycle_values,
    // so we use Option<>.
    pub issue_cycle: Option<u8>,
    pub execute_begin_cycle: Option<u8>,
    pub execute_cycle: Option<u8>,
    pub write_back_cycle: Option<u8>,
}

#[derive(Debug)]
pub struct Reservation{
    pub inner: BTreeMap<RSId,RSinner>,
}

impl Reservation {
    pub fn new()->Reservation{
        let mut inner=BTreeMap::new();
        // LD 
        for index in 0..LD_RS_COUNT{
            inner.insert(
                RSId(index, RSType::LD),
                RSinner::new(RSType::LD,index));
        }
        // SD
        for index in 0..SD_RS_COUNT{
            inner.insert(
                RSId(index, RSType::SD),
                RSinner::new(RSType::SD,index));
        }
        
        for index in 0..ADD_RS_COUNT{
            inner.insert(
                RSId(index, RSType::ADD),
                RSinner::new(RSType::ADD,index));
        }
        for index in 0..MULT_RS_COUNT{
            inner.insert(
                RSId(index, RSType::MULT),
                RSinner::new(RSType::MULT,index));
        }

        Reservation { inner }
    }

    pub fn get_free(&self, op:Type)->Option<RSId>{
        let ty:RSType=op.clone().into();

        for inner in self.inner.values() {
            if inner.op == ty && inner.state == RSState::Free{
                return Some(inner.id);
            }
        }
        None
    }

    pub fn insert(&mut self,ins:&Instruction,freg:&mut FRegFile,id:RSId,rob:&ReorderBuffer,cycle:&u8,inst_issued: &usize){
        let mut inst = ins.clone();
        inst.robid.replace(ROBID(*inst_issued));
        inst.issue_cycle.replace(*cycle);
        if let Some(rs_inner_entry) = self.inner.get_mut(&id){
            rs_inner_entry.modify(&mut inst,freg,rob,id.0,cycle);
            rs_inner_entry.inst.replace(inst);
        }  
    }

    pub fn calc(&mut self, cycle: &u8)->Vec<Instruction>{
        // change the Ready into Executing
        let mut writeback_vec = Vec::<Instruction>::new();
        self.inner.iter_mut().for_each(|rs_entry| {
            let mut entry = rs_entry.1;
            match entry.state {
                RSState::Ready => {
                    entry.state=RSState::Executing;
                    entry.execute_begin_cycle.replace(cycle.clone() as u8);
                    entry.execute_cycle.replace(1);
                    entry.inst.as_mut()
                              .unwrap()
                               .execute_begin_cycle.replace(cycle.clone() as u8);
                }
                RSState::Executing => {
                    entry.execute_cycle.replace(entry.execute_cycle.unwrap()+1);
                    if entry.is_finished() {
                        entry.state = RSState::Finished;
                        entry.inst.as_mut().unwrap().execute_end_cycle.replace(*cycle);
                    }
                }
                // write back to ROB
                RSState::Finished => {
                    entry.inst.as_mut().unwrap().write_back_cycle.replace(*cycle);
                    // calculate the result and write back
                    let op = entry.inst.as_ref().unwrap().op.clone();
                    match op {
                        Type::LD | Type::SD => {
                            let v1 = entry.vk.clone();
                            let v2 = entry.addr.clone();
                            let mut result=apply_op(Type::ADDD, v1.unwrap(), v2.unwrap());
                            result = Value::new(ValueInner::MemAddr(result));
                            entry.inst.as_mut().unwrap().set_result(result);
                        }
                        _ => {
                            let v1 = entry.vj.clone();
                            let v2 = entry.vk.clone();
                            let result=apply_op(op, v1.unwrap(), v2.unwrap());
                            entry.inst.as_mut().unwrap().set_result(result);
                        }
                    }
                    writeback_vec.push(entry.inst.as_ref().unwrap().clone());
                    entry.clear();
                }
                
                // for the begin ld
                RSState::Begin => entry.state=RSState::Ready,
                _ => {}
            }
        });
        writeback_vec
    }
}

impl RSinner {
    pub fn new(optype:RSType,id:usize)->RSinner {
        RSinner { 
            id:RSId(id, optype),
            inst:None,
            op: optype, 
            state: RSState::Free, 
            vj: None, 
            vk: None, 
            qj: None, 
            qk: None, 
            addr: None,
            result: None,
            issue_cycle: None, 
            execute_begin_cycle: None, 
            execute_cycle: None, 
            write_back_cycle: None,
        }
    }

    // modify rs from inst
    pub fn modify(&mut self,inst:&Instruction,freg:&mut FRegFile,rob:&ReorderBuffer,id:usize,cycle:&u8){
        // use clone to aviod Transferting of ownership
        self.id=RSId(id,inst.op.clone().into());
        self.op=inst.op.clone().into();
        match inst.op {
            Type::LD => {
                // because the LD instructions on the top,
                // we assume that the value is always ready
                self.addr.replace(inst.src1.as_ref().unwrap().clone());
                self.vk.replace(inst.src2.as_ref().unwrap().clone());
                self.issue_cycle.replace(*cycle);
                self.state=RSState::Begin;
                if let Unit::RF(rfid) = inst.dest{
                let reg=freg.get_mut(&rfid);
                reg.src.replace(inst.robid.unwrap().clone());
                reg.value=None;
                }
                
            },
            Type::SD => {
                self.addr.replace(inst.src1.as_ref().unwrap().clone());
                self.vj.replace(inst.src2.as_ref().unwrap().clone());
                self.issue_cycle.replace(*cycle);

                // the dest 
                if let Unit::RF(fregid)= inst.dest {
                    let reg = freg.get(&fregid);

                    match &reg.value{
                        // there is a value stored in the reg
                        Some(value)=>{
                            self.vk.replace(value.clone());
                            self.state=RSState::Ready;
                        }
                        None=>{
                            self.qk.replace(reg.src.unwrap().clone());
                            self.state=RSState::Waitting;
                        }
                    }
                }

            },
            _=>{
                if let Some(src1)=inst.src1.clone(){
                    match *src1{
                        ValueInner::Unit(Unit::RF(rfid))=>{
                            let reg = freg.get(&rfid);
                            match &reg.src {
                                Some(robid)=>{
                                    let result= rob.get_value(robid);
                                    match result{
                                        Some(value) =>{
                                            self.vj=Some(value);
                                            self.state=RSState::Ready;
                                        }
                                        None =>{                                    
                                            self.qj.replace(reg.src.unwrap().clone());
                                            self.state=RSState::Waitting;} 
                                    }
                                }
                                None=>{
                                    match &reg.value {
                                        Some(value)=>{
                                            self.vj.replace(value.clone());
                                            self.state=RSState::Ready;
                                        }
                                        None=>{
                                            panic!("src1 read reg error");
                                        }
                                    }
                                }
                            }
                        }
                        _=>{panic!("src1 error")}
                    }
                }

                if let Some(src2)=inst.src2.clone(){
                    match *src2{
                        ValueInner::Unit(Unit::RF(rfid))=>{
                            let reg = freg.get(&rfid);
                            match &reg.src {
                                Some(robid)=>{
                                    let result= rob.get_value(robid);
                                    match result{
                                        Some(value) =>{
                                            self.vk=Some(value);
                                        }
                                        None =>{                                    
                                            self.qk.replace(reg.src.unwrap().clone());
                                            self.state=RSState::Waitting;
                                        } 
                                    }
                                }
                                None=>{
                                    match &reg.value {
                                        Some(value)=>{
                                            self.vk.replace(value.clone());
                                        }
                                        None=>{
                                            panic!("src2 read reg error");
                                        }
                                    }

                                }
                            }
                        }
                        _=>{panic!("src2 error")}
                    }
                }
                if let Unit::RF(rfid) = inst.dest{
                    let reg=freg.get_mut(&rfid);
                    reg.src.replace(inst.robid.unwrap().clone());
                    reg.value=None;
                    }
                
                self.issue_cycle.replace(*cycle);
            }
        }

    }

    pub fn clear(&mut self){
        self.inst=None;
        self.state = RSState::Free;
        self.vj = None;
        self.vk = None;
        self.qj = None;
        self.qk = None;
        self.addr = None;
        self.result=None;
        self.issue_cycle=None;
        self.execute_begin_cycle=None;
        self.execute_cycle=None;
        self.write_back_cycle=None;
    }

    pub fn is_finished(&mut self)->bool{
        let mut flag:bool = false;
        if self.execute_cycle.unwrap() == self.inst.as_ref().unwrap().required_cycle {
            flag=true;
        }
        flag
    }
}

impl From<Type> for RSType{
    fn from(value: Type) -> Self {
        match value {
            Type::LD | Type::SD => RSType::LD,
            Type::ADDD | Type::SUBD => RSType::ADD,
            Type::MULTD | Type::DIVD=> RSType::MULT,
        }
    }
}

impl std::fmt::Display for Reservation{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for inner in self.inner.values(){
            write!(f, "{inner}\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for RSinner{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let entry = format!("{}{}",self.op,self.id.0);

        let op = match self.inst.as_ref(){
            Some(inst)=> format!("{}",inst.op),
            None => String::from("None "),
        };

        let vj = match self.vj.as_ref(){
            Some(value)=>style(value.to_str()).bold(),
            None=>style(String::from("None"))
        };

        let vk = match self.vk.as_ref(){
            Some(value)=>style(value.to_str()),
            None=>style(String::from("None"))
        };

        let qj = match self.qj.as_ref(){
            Some(value)=>format!("{value}"),
            None=>String::from("None")
        };

        let qk = match self.qk.as_ref(){
            Some(value)=>format!("{value}"),
            None=>String::from("None")
        };
        let addr = match self.addr.as_ref() {
            Some(v) => style(format!("{v}")).white().bright(),
            None => style(String::from("None ")).white(),
        };

        write!(f,
            "{:<5} -> {}, {}, {:10}, {:10}, {:10}, {:10}, {}",
            entry, op,self.state,vj,vk,qj,qk,addr
        )

    }
}

impl std::fmt::Display for RSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSType::ADD=> write!(f,"ADD"),
            RSType::LD=> write!(f,"LD"),
            RSType::MULT=> write!(f,"MULT"),
            RSType::SD=>write!(f,"SD")
        }
    }
}

impl std::fmt::Display for RSState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSState::Free=>write!(f,"{:6}",style(String::from("Free")).cyan().bold()),
            RSState::Executing=>write!(f,"{:6}",style(String::from("Exec")).cyan().bold()),
            RSState::Ready|RSState::Begin=>write!(f,"{:6}",style(String::from("Ready")).cyan().bold()),
            RSState::Finished=>write!(f,"{:6}",style(String::from("Finish")).cyan().bold()),
            RSState::Waitting=>write!(f,"{:6}",style(String::from("Wait")).cyan().bold()),
        }
    }
}