use core::panic;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::exit;

use crate::vm::instructions;
use crate::vm::opcode;
use crate::vm::syscall;

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

            Ok(OpCode::ExecTrap) => {
                let syscall = instructions::SysCall::from(inst);
                match syscall.trap_vect_8().try_into() {
                    Ok(syscall::SysCall::GetC) => {
                        let mut buff: [u8; 1] = [0; 1];
                        io::stdin().read(&mut buff).unwrap();
                        self.registers[Cpu::R0] = buff[0] as u16;
                        self.update_flag(Cpu::R0);
                    }
                    Ok(syscall::SysCall::Out) => {
                        let buff = [self.registers[Cpu::R0] as u8; 1];
                        io::stdout().write_all(&buff).unwrap();
                    }
                    Ok(syscall::SysCall::PutS) => {
                        let mut idx = self.registers[Cpu::R0] as usize;
                        let mut str = String::from("");
                        while mem[idx] != 0 {
                            str.push(char::from_u32(mem[idx].into()).unwrap());
                            idx += 1;
                        }
                        print!("{str}");
                        io::stdout().flush().unwrap();
                    }
                    Ok(syscall::SysCall::In) => {
                        print!("input: ");
                        io::stdout().flush().unwrap();

                        let mut buff: [u8; 1] = [0; 1];
                        io::stdin().read(&mut buff).unwrap();
                        self.registers[Cpu::R0] = buff[0] as u16;
                        self.update_flag(Cpu::R0);
                    }
                    Ok(syscall::SysCall::PutSp) => {
                        let mut idx = self.registers[Cpu::R0] as usize;
                        let mut str = String::from("");
                        while mem[idx] != 0 {
                            let low = (mem[idx] & 0xFF) as u8;
                            let high = ((mem[idx] >> 8) & 0xFF) as u8;
                            str.push(char::from_u32(low.into()).unwrap());
                            str.push(char::from_u32(high.into()).unwrap());
                            idx += 1;
                        }
                        io::stdout().write_all(&str.as_bytes()).unwrap();
                    }
                    Ok(syscall::SysCall::Halt) => {
                        println!("HALT");
                        exit(0);
                    }
                    Err(_) => panic!("bad syscall"),
                }
            }

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

const FL_POS: u16 = 1 << 0;
const FL_ZERO: u16 = 1 << 1;
const FL_NEG: u16 = 1 << 2;

const PC_START: u16 = 0x3000;
