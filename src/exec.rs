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

    fn next_number(&mut self) -> Result<Number, Box<dyn Error>> {
        let number = self.data.get(self.ptr);

        self.ptr += 1;

        match number {
            Some(num) => Ok(num.clone()),
            None => Err(format!("Attempt to read out of the executable code.").into()),
        }
    }

    fn next_two_numbers(&mut self) -> Result<(Number, Number), Box<dyn Error>> {
        let a = self.next_number()?;
        let b = self.next_number()?;

        Ok((a, b))
    }

    fn next_three_numbers(&mut self) -> Result<(Number, Number, Number), Box<dyn Error>> {
        let a = self.next_number()?;
        let b = self.next_number()?;
        let c = self.next_number()?;

        Ok((a, b, c))
    }

    pub fn next_instruction(&mut self) -> Result<INSTRUCTION, Box<dyn Error>> {
        let instruction = match self.next_number()?.raw_value() {
            0 => INSTRUCTION::HALT,
            1 => {
                let (a, b) = self.next_two_numbers()?;
                a.assert_register()?;
                b.assert_literal()?;
                INSTRUCTION::SET(a, b)
            }
            6 => {
                let a = self.next_number()?;
                INSTRUCTION::JMP(a)
            },
            7 => {
                let (a, b) = self.next_two_numbers()?;
                INSTRUCTION::JT(a, b)
            },
            8 => {
                let (a, b) = self.next_two_numbers()?;
                INSTRUCTION::JF(a, b)
            },
            19 => {
                let a = self.next_number()?;
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
