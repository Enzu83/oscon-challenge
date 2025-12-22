use std::error::Error;

use crate::{exec::Executable, inst::INSTRUCTION, mem::Memory, num::Number};

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
            INSTRUCTION::SET(a, b) => {
                let idx = a.value() as usize;
                let value = self.mem_value(&b)?;
                self.memory.write_register(idx, value)?;
            }
            INSTRUCTION::JMP(a) => {
                executable.jump_to(a.value())?;
            },
            INSTRUCTION::JT(a, b) => {
                if self.mem_value(&a)? != 0 {
                    executable.jump_to(b.value())?;
                }
            },
            INSTRUCTION::JF(a, b) => {
                if self.mem_value(&a)? == 0 {
                    executable.jump_to(b.value())?;
                }
            },
            INSTRUCTION::OUT(a) => {
                print!("{}", self.mem_value(&a)? as u8 as char);
            },
            INSTRUCTION::NOOP => {},
            _ => {},
        }

        Ok(())
    }

    pub fn mem_value(&self, number: &Number) -> Result<u16, Box<dyn Error>> {
        if number.is_literal() {
            return Ok(number.value());
        }

        if number.is_valid() {
            return self.memory.read_register(number.value() as usize);
        }

        Err(format!("Can't get value of {:?}", number).into())
    }
}
