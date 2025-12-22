use std::{error::Error, fs::read};

use crate::num::Number;
use crate::inst::INSTRUCTION;

pub struct Executable {
    data: Vec<Number>,
    ptr: usize,
    running: bool,
}

impl Executable {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let raw = read(path)?;
        let data = Number::from_hex_slice(&raw)?;

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
            return Err(format!("Attempt to read out of the executable code.").into());
        }

        self.ptr = address as usize;

        Ok(())
    }

    pub fn get_next(&self) -> Result<Number, Box<dyn Error>> {
        match self.data.get(self.ptr) {
            Some(num) => Ok(num.clone()),
            None => Err(format!("Attempt to read out of the executable code.").into()),
        }
    }

    pub fn go_to_next(&mut self) {
        self.ptr +=1;
    }

    pub fn next(&mut self) -> Result<Number, Box<dyn Error>> {
        let number = self.get_next()?;
        self.go_to_next();
        
        Ok(number)
    }

    fn next_two(&mut self) -> Result<(Number, Number), Box<dyn Error>> {
        let a = self.next()?;
        let b = self.next()?;

        Ok((a, b))
    }

    fn next_three(&mut self) -> Result<(Number, Number, Number), Box<dyn Error>> {
        let a = self.next()?;
        let b = self.next()?;
        let c = self.next()?;

        Ok((a, b, c))
    }

    pub fn next_instruction(&mut self) -> Result<INSTRUCTION, Box<dyn Error>> {
        let instruction = match self.next()?.raw_value() {
            0 => INSTRUCTION::HALT,
            1 => {
                let (a, b) = self.next_two()?;
                a.assert_register()?;
                b.assert_literal()?;
                INSTRUCTION::SET(a, b)
            },
            2 => {
                let a = self.next()?;
                INSTRUCTION::PUSH(a)
            },
            3 => {
                let a = self.next()?;
                a.assert_register()?;
                INSTRUCTION::POP(a)
            },
            4 => {
                let (a, b, c) = self.next_three()?;
                a.assert_register()?;
                INSTRUCTION::EQ(a, b, c)
            },
            5 => {
                let (a, b, c) = self.next_three()?;
                a.assert_register()?;
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
                a.assert_register()?;
                INSTRUCTION::ADD(a, b, c)
            },
            12 => {
                let (a, b, c) = self.next_three()?;
                a.assert_register()?;
                INSTRUCTION::AND(a, b, c)
            },
            13 => {
                let (a, b, c) = self.next_three()?;
                a.assert_register()?;
                INSTRUCTION::OR(a, b, c)
            },
            14 => {
                let (a, b) = self.next_two()?;
                a.assert_register()?;
                INSTRUCTION::NOT(a, b)
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

        Ok(instruction)
    }
}
