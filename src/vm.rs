use std::error::Error;

use crate::{exec::Executable, inst::INSTRUCTION, mem::Memory};

pub struct VM {
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        Self { memory: Memory::new() }
    }

    pub fn run(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut executable = Executable::new(path)?;
        
        while executable.is_running() {
            self.execute_instruction(&mut executable)?;
        };

        Ok(())
    }

    pub fn execute_instruction(&mut self, executable: &mut Executable) -> Result<(), Box<dyn Error>> {
        match executable.next_instruction()? {
            INSTRUCTION::HALT => {
                executable.stop();
                //self.memory.dump()?;
            },
            INSTRUCTION::PUSH(a) => {
                let value = self.memory.read(a)?;
                self.memory.push_stack(value);
            },
            INSTRUCTION::POP(a) => {
                let idx = a;
                let value = self.memory.pop_stack()?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::SET(a, b) => {
                let idx = a;
                let value = self.memory.read(b)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::EQ(a, b, c) => {
                let idx = a;
                let value = (self.memory.read(b)? == self.memory.read(c)?) as u16;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::GT(a, b, c) => {
                let idx = a;
                let value = (self.memory.read(b)? > self.memory.read(c)?) as u16;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::JMP(a) => {
                executable.jump_to(a as usize)?;
            },
            INSTRUCTION::JT(a, b) => {
                if self.memory.read(a)? != 0 {
                    executable.jump_to(b as usize)?;
                }
            },
            INSTRUCTION::JF(a, b) => {
                if self.memory.read(a)? == 0 {
                    executable.jump_to(b as usize)?;
                }
            },
            INSTRUCTION::ADD(a, b, c) => {
                let idx = a;
                let value = (self.memory.read(b)? + self.memory.read(c)?) % 32768;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::MULT(a, b, c) => {
                let idx = a;
                let value = (self.memory.read(b)? as u32 * self.memory.read(c)? as u32) as u16 % 32768;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::MOD(a, b, c) => {
                let idx = a;
                let value = self.memory.read(b)? % self.memory.read(c)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::AND(a, b, c) => {
                let idx = a;
                let value = self.memory.read(b)? & self.memory.read(c)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::OR(a, b, c) => {
                let idx = a;
                let value = self.memory.read(b)? | self.memory.read(c)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::NOT(a, b) => {
                let idx = a;
                let value = !self.memory.read(b)? % 32768;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::RMEM(a, b) => {
                let b_address = self.memory.read(b)? as usize;
                let value = executable.read_at(b_address)?;
                let idx = a;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::CALL(a) => {
                let next_inst_addr = executable.current();
                self.memory.push_stack(next_inst_addr);
                executable.jump_to(self.memory.read(a)? as usize)?;
            },
            INSTRUCTION::OUT(a) => {
                print!("{}", a as u8 as char);
            },
            INSTRUCTION::NOOP => {},
            _ => {},
        }

        Ok(())
    }
}
