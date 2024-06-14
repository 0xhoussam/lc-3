use std::convert::TryFrom;

pub enum SysCall {
    GetC = 0x20,
    Out = 0x21,
    PutS = 0x22,
    In = 0x23,
    PutSp = 0x24,
    Halt = 0x25,
}

impl TryFrom<usize> for SysCall {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == SysCall::GetC as usize => Ok(SysCall::GetC),
            x if x == SysCall::Out as usize => Ok(SysCall::Out),
            x if x == SysCall::PutS as usize => Ok(SysCall::PutS),
            x if x == SysCall::In as usize => Ok(SysCall::In),
            x if x == SysCall::PutSp as usize => Ok(SysCall::PutSp),
            x if x == SysCall::Halt as usize => Ok(SysCall::Halt),
            _ => Err(()),
        }
    }
}
