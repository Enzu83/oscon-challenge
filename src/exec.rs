use std::{error::Error, fs::read};

use crate::num::from_hex_slice;
use crate::inst::INSTRUCTION;

pub struct Executable {
    data: Vec<u16>,
    ptr: usize,
    running: bool,
}

impl Executable {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let raw = read(path)?;
        let data = from_hex_slice(&raw)?;

        Ok(Self { data, ptr: 0, running: true })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn jump_to(&mut self, address: u16) -> Result<(), Box<dyn Error>> {
        if address as usize >= self.data.len() {
            return Err(format!("Attempt to jump outside of the executable code boundaries ({})", address).into());
        }

        self.ptr = address as usize;

        Ok(())
    }

    pub fn current_address(&self) -> u16 {
        self.ptr as u16
    }

    pub fn next(&mut self) -> Result<u16, Box<dyn Error>> {
        let number = match self.data.get(self.ptr) {
            Some(num) => Ok(num.clone()),
            None => Err(format!("Attempt to read outside of the executable code boundaries ({})", self.ptr).into()),
        };
        
        self.ptr += 1;
        
        return number;
    }

    fn next_two(&mut self) -> Result<(u16, u16), Box<dyn Error>> {
        let a = self.next()?;
        let b = self.next()?;

        Ok((a, b))
    }

    fn next_three(&mut self) -> Result<(u16, u16, u16), Box<dyn Error>> {
        let a = self.next()?;
        let b = self.next()?;
        let c = self.next()?;

        Ok((a, b, c))
    }

    pub fn next_instruction(&mut self) -> Result<INSTRUCTION, Box<dyn Error>> {
        let instruction = match self.next()? {
            0 => INSTRUCTION::HALT,
            1 => {
                let (a, b) = self.next_two()?;
                INSTRUCTION::SET(a, b)
            },
            2 => {
                let a = self.next()?;
                INSTRUCTION::PUSH(a)
            },
            3 => {
                let a = self.next()?;
                INSTRUCTION::POP(a)
            },
            4 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::EQ(a, b, c)
            },
            5 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::GT(a, b, c)
            },
            6 => {
                let a = self.next()?;
                INSTRUCTION::JMP(a)
            },
            7 => {
                let (a, b) = self.next_two()?;
                INSTRUCTION::JT(a, b)
            },
            8 => {
                let (a, b) = self.next_two()?;
                INSTRUCTION::JF(a, b)
            },
            9 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::ADD(a, b, c)
            },
            10 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::MULT(a, b, c)
            },
            11 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::MOD(a, b, c)
            },
            12 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::AND(a, b, c)
            },
            13 => {
                let (a, b, c) = self.next_three()?;
                INSTRUCTION::OR(a, b, c)
            },
            14 => {
                let (a, b) = self.next_two()?;
                INSTRUCTION::NOT(a, b)
            },
            15 => {
                let (a, b) = self.next_two()?;
                INSTRUCTION::RMEM(a, b)
            },
            17 => {
                let a = self.next()?;
                INSTRUCTION::CALL(a)
            },
            19 => {
                let a = self.next()?;
                INSTRUCTION::OUT(a)
            },
            21 => INSTRUCTION::NOOP,
            opcode => {
                return Err(format!("Invalid opcode: {}", opcode).into())
            },
        };

        //println!("{:?}", instruction);

        Ok(instruction)
    }
}
