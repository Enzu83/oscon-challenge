use std::{error::Error, fs::read};

use crate::{inst::INSTRUCTION, mem::Memory, num::from_hex_slice, proc::Process};

pub struct VM {
    memory: Memory,
    process: Process,
}

impl VM {
    pub fn new() -> Self {
        Self { memory: Memory::new(), process: Process::new() }
    }

    pub fn execute(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let raw = read(path)?;
        let data = from_hex_slice(&raw)?;
        self.memory.load(&data)?;
        self.process.start();
        
        while self.process.is_running() {
            self.next_instruction()?;
        };

        Ok(())
    }

    fn read(&self, address: u16) -> Result<u16, Box<dyn Error>> {
        self.memory.read(address)
    }

    fn read_and_increment(&mut self) -> Result<u16, Box<dyn Error>> {
        let value = self.read(self.process.address())?;
        self.process.jump_to_next();
        Ok(value)
    }

    fn read_instruction(&mut self) -> Result<INSTRUCTION, Box<dyn Error>> {
        let instruction = match self.read_and_increment()? {
            0 => INSTRUCTION::HALT,
            1 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                INSTRUCTION::SET(a, b)
            },
            2 => {
                let a = self.read_and_increment()?;
                INSTRUCTION::PUSH(a)
            },
            3 => {
                let a = self.read_and_increment()?;
                INSTRUCTION::POP(a)
            },
            4 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::EQ(a, b, c)
            },
            5 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::GT(a, b, c)
            },
            6 => {
                let a = self.read_and_increment()?;
                INSTRUCTION::JMP(a)
            },
            7 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                INSTRUCTION::JT(a, b)
            },
            8 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                INSTRUCTION::JF(a, b)
            },
            9 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::ADD(a, b, c)
            },
            10 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::MULT(a, b, c)
            },
            11 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::MOD(a, b, c)
            },
            12 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::AND(a, b, c)
            },
            13 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                let c = self.read_and_increment()?;
                INSTRUCTION::OR(a, b, c)
            },
            14 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                INSTRUCTION::NOT(a, b)
            },
            15 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                INSTRUCTION::RMEM(a, b)
            },
            16 => {
                let a = self.read_and_increment()?;
                let b = self.read_and_increment()?;
                INSTRUCTION::WMEM(a, b)
            },
            17 => {
                let a = self.read_and_increment()?;
                INSTRUCTION::CALL(a)
            },
            19 => {
                let a = self.read_and_increment()?;
                INSTRUCTION::OUT(a)
            },
            21 => INSTRUCTION::NOOP,
            opcode => {
                return Err(format!("Invalid opcode: {}", opcode).into())
            },
        };

        println!("{:?}", instruction);

        Ok(instruction)
    }

    pub fn next_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        match self.read_instruction()? {
            INSTRUCTION::HALT => {
                self.process.stop();
                //self.memory.dump()?;
            },
            INSTRUCTION::PUSH(a) => {
                let value = self.memory.read(a)?;
                self.memory.push_stack(value);
            },
            INSTRUCTION::POP(a) => {
                let value = self.memory.pop_stack()?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::SET(a, b) => {
                let value = self.memory.value(b)?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::EQ(a, b, c) => {
                let value = (self.memory.value(b)? == self.memory.value(c)?) as u16;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::GT(a, b, c) => {
                let value = (self.memory.value(b)? > self.memory.value(c)?) as u16;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::JMP(a) => {
                self.process.jump_to(a);
            },
            INSTRUCTION::JT(a, b) => {
                if self.memory.value(a)? != 0 {
                    self.process.jump_to(b);
                }
            },
            INSTRUCTION::JF(a, b) => {
                if self.memory.value(a)? == 0 {
                    self.process.jump_to(b);
                }
            },
            INSTRUCTION::ADD(a, b, c) => {
                let value = (self.memory.value(b)? + self.memory.value(c)?) % 32768;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::MULT(a, b, c) => {
                let value = (self.memory.value(b)? as u32 * self.memory.value(c)? as u32) as u16 % 32768;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::MOD(a, b, c) => {
                let value = self.memory.value(b)? % self.memory.value(c)?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::AND(a, b, c) => {
                let value = self.memory.value(b)? & self.memory.value(c)?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::OR(a, b, c) => {
                let value = self.memory.value(b)? | self.memory.value(c)?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::NOT(a, b) => {
                let value = !self.memory.value(b)? % 32768;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::RMEM(a, b) => {
                let b_address = self.memory.value(b)?;
                let value = self.memory.read(b_address)?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::WMEM(a, b) => {
                let value = self.memory.value(b)?;
                self.memory.write(a, value)?;
            },
            INSTRUCTION::CALL(a) => {
                let next_inst_addr = self.process.address();
                self.memory.push_stack(next_inst_addr);
                self.process.jump_to(self.memory.value(a)?);
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
