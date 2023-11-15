pub mod rs;
pub mod fu;
pub mod register;
pub mod rob;

pub use rs::*;
pub use fu::*;
pub use register::*;
pub use rob::*;

pub enum Unit {
    Rob(ROBID),

    RF(RFID),

    Reg(RegID)
}