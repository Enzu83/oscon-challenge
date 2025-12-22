use std::process::exit;
use std::{error::Error, fs::read};

use crate::number::from_hex_slice;
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

    pub fn next_number(&mut self) -> Result<u16, Box<dyn Error>> {
        self.ptr += 1;

        match self.data.get(self.ptr)  {
            Some(value) => Ok(*value),
            None => Err(format!("Attempt to read out of the executable code.").into()),
        }
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
            19 => {
                let a = self.next_number()?;
                INSTRUCTION::OUT(a)
            },
            21 => INSTRUCTION::NOOP,
            opcode => return Err(format!("Opcode {} not implemented yet.", opcode).into()),
        };

        Ok(instruction)
    }

    fn exec_instruction(&mut self, instruction: INSTRUCTION) -> Result<(), Box<dyn Error>> {
        match instruction {
            INSTRUCTION::HALT => {
                exit(0);
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
