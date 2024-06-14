use crate::vm::opcode::OpCode;
use bitfield_struct::bitfield;

#[bitfield(u16)]
pub struct Store {
    #[bits(9)]
    pub pc_offset_9: usize,

    #[bits(3)]
    pub src: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct StoreIndirect {
    #[bits(9)]
    pub pc_offset_9: usize,

    #[bits(3)]
    pub src: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct StoreRegister {
    #[bits(6)]
    pub pc_offset_6: usize,

    #[bits(3)]
    pub base_register: usize,

    #[bits(3)]
    pub src: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct Add {
    #[bits(3)]
    pub src2: usize,

    #[bits(3)]
    padding: usize,

    #[bits(3)]
    pub src1: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    opcode: OpCode,
}

#[bitfield(u16)]
pub struct ImmediateAdd {
    #[bits(5)]
    pub imm5: usize,

    pub flag: bool,

    #[bits(3)]
    pub src: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct LoadIndirect {
    #[bits(9)]
    pub pc_offset_9: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct ImmediateAnd {
    #[bits(5)]
    pub imm5: usize,

    pub flag: bool,

    #[bits(3)]
    pub src: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct And {
    #[bits(3)]
    pub src2: usize,

    #[bits(3)]
    padding: usize,

    #[bits(3)]
    pub src1: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}
#[bitfield(u16)]
pub struct Branch {
    #[bits(9)]
    pub pc_offset_9: usize,

    pub p: bool,
    pub z: bool,
    pub n: bool,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct Jump {
    #[bits(6)]
    padding: usize,

    #[bits(3)]
    pub base_register: usize,

    #[bits(3)]
    padding2: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct JumpSubRoutine {
    #[bits(11)]
    pub pc_offset_11: usize,

    pub flag: bool,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct JumpSubRoutineRegister {
    #[bits(6)]
    padding: usize,

    #[bits(3)]
    pub base_register: usize,

    #[bits(3)]
    padding1: usize,

    #[bits(4)]
    pub opcode: OpCode,
}
#[bitfield(u16)]
pub struct Load {
    #[bits(9)]
    pub pc_offset_9: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct LoadRegister {
    #[bits(6)]
    pub pc_offset_6: usize,

    #[bits(3)]
    pub base_register: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct Lea {
    #[bits(9)]
    pub pc_offset_9: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct Not {
    #[bits(6)]
    padding: usize,

    #[bits(3)]
    pub src: usize,

    #[bits(3)]
    pub dest: usize,

    #[bits(4)]
    pub opcode: OpCode,
}

#[bitfield(u16)]
pub struct SysCall {
    #[bits(8)]
    pub trap_vect_8: usize,

    #[bits(4)]
    padding: usize,

    #[bits(4)]
    pub opcode: OpCode,
}
