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
            },
            INSTRUCTION::PUSH(a) => {
                let value = a.value(&self.memory)?;
                self.memory.push_stack(value);
            },
            INSTRUCTION::POP(a) => {
                let idx = a.raw_value();
                let value = self.memory.pop_stack()?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::SET(a, b) => {
                let idx = a.raw_value();
                let value = b.value(&self.memory)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::EQ(a, b, c) => {
                let idx = a.raw_value();
                let value = (b.value(&self.memory)? == c.value(&self.memory)?) as u16;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::GT(a, b, c) => {
                let idx = a.raw_value();
                let value = (b.value(&self.memory)? > c.value(&self.memory)?) as u16;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::JMP(a) => {
                executable.jump_to(a.raw_value())?;
            },
            INSTRUCTION::JT(a, b) => {
                if a.value(&self.memory)? != 0 {
                    executable.jump_to(b.raw_value())?;
                }
            },
            INSTRUCTION::JF(a, b) => {
                if a.value(&self.memory)? == 0 {
                    executable.jump_to(b.raw_value())?;
                }
            },
            INSTRUCTION::ADD(a, b, c) => {
                let idx = a.raw_value();
                let value = (b.value(&self.memory)? + c.value(&self.memory)?) % 32768;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::MULT(a, b, c) => {
                let idx = a.raw_value();
                let value = (b.value(&self.memory)? as u32 * c.value(&self.memory)? as u32) as u16 % 32768;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::MOD(a, b, c) => {
                let idx = a.raw_value();
                let value = b.value(&self.memory)? % c.value(&self.memory)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::AND(a, b, c) => {
                let idx = a.raw_value();
                let value = b.value(&self.memory)? & c.value(&self.memory)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::OR(a, b, c) => {
                let idx = a.raw_value();
                let value = b.value(&self.memory)? | c.value(&self.memory)?;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::NOT(a, b) => {
                let idx = a.raw_value();
                let value = (!b.value(&self.memory)? << 1) >> 1;
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::RMEM(a, b) => {
                let b_idx = b.raw_value();
                let value = self.memory.read(b_idx)?;
                
                let idx = a.raw_value();
                self.memory.write(idx, value)?;
            },
            INSTRUCTION::CALL(a) => {
                let next_inst_addr = executable.current_address();
                self.memory.push_stack(next_inst_addr);
                executable.jump_to(a.value(&self.memory)?)?;
            },
            INSTRUCTION::OUT(a) => {
                print!("{}", a.raw_value() as u8 as char);
            },
            INSTRUCTION::NOOP => {},
            _ => {},
        }

        Ok(())
    }
}
