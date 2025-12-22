use std::process::exit;
use std::{error::Error, fs::read};

use crate::num::from_hex_slice;
use crate::instr::INSTRUCTION;

pub struct Executable {
    data: Vec<u16>,
    ptr: usize,
}

impl Executable {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let raw = read(path)?;
        let data = from_hex_slice(&raw)?;

        Ok(Self { data, ptr: 0 })
    }

    fn next_number(&mut self) -> Result<u16, Box<dyn Error>> {
        let number = self.data.get(self.ptr);

        self.ptr += 1;

        match number {
            Some(value) => Ok(*value),
            None => Err(format!("Attempt to read out of the executable code.").into()),
        }
    }

    fn next_two_numbers(&mut self) -> Result<(u16, u16), Box<dyn Error>> {
        let a = self.next_number()?;
        let b = self.next_number()?;

        Ok((a, b))
    }

    fn next_three_numbers(&mut self) -> Result<(u16, u16, u16), Box<dyn Error>> {
        let a = self.next_number()?;
        let b = self.next_number()?;
        let c = self.next_number()?;

        Ok((a, b, c))
    }

    pub fn exec(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let instruction = self.next_instruction()?;
            self.exec_instruction(instruction)?;
        }
    }

    fn next_instruction(&mut self) -> Result<INSTRUCTION, Box<dyn Error>> {
        let instruction = match self.next_number()? {
            0 => INSTRUCTION::HALT,
            6 => {
                let a = self.next_number()?;
                INSTRUCTION::JMP(a)
            }
            19 => {
                let a = self.next_number()?;
                INSTRUCTION::OUT(a)
            },
            21 => INSTRUCTION::NOOP,
            opcode => return Err(format!("Invalid opcode {}", opcode).into()),
        };

        Ok(instruction)
    }

    fn exec_instruction(&mut self, instruction: INSTRUCTION) -> Result<(), Box<dyn Error>> {
        match instruction {
            INSTRUCTION::HALT => {
                exit(0);
            },
            INSTRUCTION::JMP(a) => {
                self.ptr = a as usize;
            }
            INSTRUCTION::OUT(a) => {
                print!("{}", a as u8 as char);
            },
            INSTRUCTION::NOOP => {},
            _ => {},
        }

        Ok(())
    }
}
