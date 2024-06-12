use std::convert::TryFrom;

#[derive(Debug)]
pub enum OpCode {
    Branch,
    Add,
    Load,
    Store,
    JumpRegister,
    And,
    LoadRegister,
    StoreRegister,
    Rti,
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    Reserved,
    Lea,
    ExecTrap,
}

impl OpCode {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(value: u16) -> Self {
        match value {
            0 => OpCode::Branch,
            1 => OpCode::Add,
            2 => OpCode::Load,
            3 => OpCode::Store,
            4 => OpCode::JumpRegister,
            5 => OpCode::And,
            6 => OpCode::LoadRegister,
            7 => OpCode::StoreRegister,
            8 => OpCode::Rti,
            9 => OpCode::Not,
            10 => OpCode::LoadIndirect,
            11 => OpCode::StoreIndirect,
            12 => OpCode::Jump,
            13 => OpCode::Reserved,
            14 => OpCode::Lea,
            15 => OpCode::ExecTrap,
            _ => panic!("bad bit value"),
        }
    }
}

impl TryFrom<u16> for OpCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::Branch as u16 => Ok(OpCode::Branch),
            x if x == OpCode::Add as u16 => Ok(OpCode::Add),
            x if x == OpCode::Load as u16 => Ok(OpCode::Load),
            x if x == OpCode::Store as u16 => Ok(OpCode::Store),
            x if x == OpCode::JumpRegister as u16 => Ok(OpCode::JumpRegister),
            x if x == OpCode::And as u16 => Ok(OpCode::And),
            x if x == OpCode::LoadRegister as u16 => Ok(OpCode::LoadRegister),
            x if x == OpCode::StoreRegister as u16 => Ok(OpCode::StoreRegister),
            x if x == OpCode::Rti as u16 => Ok(OpCode::Rti),
            x if x == OpCode::Not as u16 => Ok(OpCode::Not),
            x if x == OpCode::LoadIndirect as u16 => Ok(OpCode::LoadIndirect),
            x if x == OpCode::StoreIndirect as u16 => Ok(OpCode::StoreIndirect),
            x if x == OpCode::Jump as u16 => Ok(OpCode::Jump),
            x if x == OpCode::Reserved as u16 => Ok(OpCode::Reserved),
            x if x == OpCode::Lea as u16 => Ok(OpCode::Lea),
            x if x == OpCode::ExecTrap as u16 => Ok(OpCode::ExecTrap),
            _ => Err(()),
        }
    }
}
