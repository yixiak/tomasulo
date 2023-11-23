pub mod rs;
pub mod register;
pub mod rob;
use std::str::FromStr;

pub use rs::*;
pub use register::*;
pub use rob::*;

#[derive(Debug,Clone,Copy,PartialEq, Eq, Hash)]
pub enum Unit {
    // Reorder Buffer item
    Rob(ROBID),

    // Floatpoint register item
    RF(RFID),

    // Immediate register item
    Reg(RegID)
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // as_bytes().get()  returns Option<&u8> 
        // so use Some(b'R') to match.
        // .chars().next() returns Option<char>  
        match s.as_bytes().get(0) {
            Some(b'R') => {
                if let Ok(id)=s[1..].parse::<u8>(){
                    Ok(Unit::Reg(RegID::new(id)))
                }else{
                    Err(())
                }
            }
            Some(b'F') => {
                if let Ok(id)=s[1..].parse::<u8>(){
                    Ok(Unit::RF(RFID::new(id)))
                }else{
                    Err(())
                }
            }
            _=> Err(())
        }
    }
}

impl From<RFID> for Unit{
    fn from(value: RFID) -> Self {
        Unit::RF(value)
    }
}

impl From<RegID> for Unit {
    fn from(value: RegID) -> Self {
        Unit::Reg(value)
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Unit::RF(id)=>write!(f,"{id}"),
            Unit::Reg(id)=>write!(f,"{id}"),
            Unit::Rob(id)=>write!(f,"{id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tomasulo_sim::Unit;

    #[test]
    fn unit_fromstr(){
        let r:Result<Unit, _>="R2".parse();
        match r {
            Ok(u) => println!("{}",u),
            Err(_)=>panic!("Error"),
        }
    }
}