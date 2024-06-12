use crate::vm::instructions;
use crate::vm::opcode;
pub struct Cpu {
    pub registers: [u16; 10],
}

impl Cpu {
    pub const R0: usize = 0;
    pub const R1: usize = 1;
    pub const R2: usize = 2;
    pub const R3: usize = 3;
    pub const R4: usize = 4;
    pub const R5: usize = 5;
    pub const R6: usize = 6;
    pub const R7: usize = 7;
    pub const PC: usize = 8;
    pub const COND: usize = 9;
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu { registers: [0; 10] };
        cpu.registers[Cpu::PC] = PC_START;
        cpu.registers[Cpu::COND] = FL_ZERO;
        return cpu;
    }

    pub fn exec_instruction(&mut self, inst: u16, mem: &mut [u16; 1 << 16]) {
        use instructions::*;
        use opcode::OpCode;
        let opcode = inst >> 12;

        match opcode.try_into() {
            Ok(OpCode::Add) => {
                let add = ImmediateAdd::from(inst);
                if !add.flag() {
                    let add = Add::from(inst);
                    self.registers[add.dest()] =
                        self.registers[add.src1()] + self.registers[add.src2()];

                    self.update_flag(add.dest());
                    return;
                }
                self.registers[add.dest()] =
                    self.registers[add.src()] + Cpu::sign_extend(add.imm5().try_into().unwrap(), 5);

                self.update_flag(add.dest());
            }

            Ok(OpCode::LoadIndirect) => {
                let load_ind = LoadIndirect::from(inst);
                let idx = Cpu::sign_extend(load_ind.pc_offset_9().try_into().unwrap(), 9)
                    + self.registers[Cpu::PC];
                self.registers[load_ind.dest()] = mem[mem[idx as usize] as usize];
            }

            Ok(OpCode::And) => {
                let and = ImmediateAnd::from(inst);
                if !and.flag() {
                    let and = And::from(inst);
                    self.registers[and.dest()] =
                        self.registers[and.src1()] & self.registers[and.src2()];
                    self.update_flag(and.dest());
                    return;
                }

                self.registers[and.dest()] =
                    self.registers[and.src()] & Cpu::sign_extend(and.imm5().try_into().unwrap(), 5);
                self.update_flag(and.dest());
            }

            Ok(OpCode::Branch) => {
                let branch = Branch::from(inst);
                if branch.p() || branch.n() || branch.z() {
                    self.registers[Cpu::PC] =
                        Cpu::sign_extend(branch.pc_offset_9().try_into().unwrap(), 9);
                }
            }

            Ok(OpCode::Jump) => {
                let jump = Jump::from(inst);
                self.registers[Cpu::PC] = self.registers[jump.base_register()];
            }

            Ok(OpCode::JumpRegister) => {
                let jump_subroutine = JumpSubRoutine::from(inst);

                self.registers[Cpu::R7] = self.registers[Cpu::PC];
                if !jump_subroutine.flag() {
                    let jump_subroutine = JumpSubRoutineRegister::from(inst);
                    self.registers[Cpu::PC] = self.registers[jump_subroutine.base_register()];
                    return;
                }
                self.registers[Cpu::PC] =
                    Cpu::sign_extend(jump_subroutine.pc_offset_11().try_into().unwrap(), 11);
            }

            Ok(OpCode::Load) => {
                let load = Load::from(inst);
                let idx = self.registers[Cpu::PC]
                    + Cpu::sign_extend(load.pc_offset_9().try_into().unwrap(), 9);
                self.registers[load.dest()] = mem[idx as usize];

                self.update_flag(load.dest());
            }

            Ok(OpCode::LoadRegister) => {
                let load_register = LoadRegister::from(inst);
                let idx = self.registers[load_register.base_register()]
                    + Cpu::sign_extend(load_register.pc_offset_6().try_into().unwrap(), 6);
                self.registers[load_register.dest()] = mem[idx as usize];
                self.update_flag(load_register.dest());
            }

            Ok(OpCode::Lea) => {
                let lea = Lea::from(inst);
                let idx = self.registers[Cpu::PC]
                    + Cpu::sign_extend(lea.pc_offset_9().try_into().unwrap(), 9);
                self.registers[lea.dest()] = mem[idx as usize];
                self.update_flag(lea.dest());
            }

            Ok(OpCode::Not) => {
                let not = Not::from(inst);
                self.registers[not.dest()] = !self.registers[not.src()];
                self.update_flag(not.dest());
            }

            Ok(OpCode::Rti) => todo!("i dont know how to handle this"),

            Ok(OpCode::Reserved) => panic!("bad upcode"),

            Ok(OpCode::Store) => {
                let store = Store::from(inst);
                let idx = self.registers[Cpu::PC]
                    + Cpu::sign_extend(store.pc_offset_9().try_into().unwrap(), 9);
                mem[idx as usize] = self.registers[store.src() as usize];
            }

            Ok(OpCode::StoreIndirect) => {
                let store_indirect = Store::from(inst);
                let idx = self.registers[Cpu::PC]
                    + Cpu::sign_extend(store_indirect.pc_offset_9().try_into().unwrap(), 9);
                mem[mem[idx as usize] as usize] = self.registers[store_indirect.src() as usize];
            }

            Ok(OpCode::StoreRegister) => {
                let store_register = StoreRegister::from(inst);
                let idx = self.registers[store_register.base_register()]
                    + Cpu::sign_extend(store_register.pc_offset_6().try_into().unwrap(), 6);
                mem[idx as usize] = self.registers[store_register.src()];
            }

            Ok(OpCode::ExecTrap) => {}

            Err(_) => panic!("bad opcode"),
        }
    }

    fn sign_extend(mut num: u16, bit_count: u16) -> u16 {
        if num >> (bit_count - 1) & 1 == 1 {
            num |= 0xFFFF << bit_count
        }
        num
    }

    fn update_flag(&mut self, reg: usize) {
        if self.registers[reg] == 0 {
            self.registers[Cpu::COND] = FL_ZERO;
        } else if self.registers[reg] >> 15 == 1 {
            self.registers[Cpu::COND] = FL_NEG;
        } else {
            self.registers[Cpu::COND] = FL_POS;
        }
    }
}

enum ConditionFlags {
    Negative,
    Zero,
    Positive,
}

const FL_POS: u16 = 1 << 0;
const FL_ZERO: u16 = 1 << 1;
const FL_NEG: u16 = 1 << 2;

const PC_START: u16 = 0x3000;

enum SysCalls {
    GetC = 0x20,
    Out = 0x21,
    PutS = 0x22,
    In = 0x23,
    PutSp = 0x24,
    Halt = 0x25,
}
