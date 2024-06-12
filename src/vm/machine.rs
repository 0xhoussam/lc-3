use crate::vm::cpu::Cpu;

pub struct Machine {
    cpu: Cpu,
    mem: [u16; 1 << 16],
}

impl Machine {
    pub fn new() -> Machine {
        let machine = Machine {
            cpu: Cpu::new(),
            mem: [0; 1 << 16],
        };
        return machine;
    }

    pub fn exec(&mut self) {
        let index = self.cpu.registers[Cpu::PC] as usize;
        let inst = self.mem.get(index).unwrap().clone();
        loop {
            self.cpu.exec_instruction(inst, &mut self.mem);
        }
    }
}
