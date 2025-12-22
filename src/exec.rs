use std::process::exit;
use std::{error::Error, fs::read};

use crate::num::Number;
use crate::instr::INSTRUCTION;

pub struct Executable {
    data: Vec<Number>,
    ptr: usize,
}

impl Executable {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let raw = read(path)?;
        let data = Number::from_hex_slice(&raw)?;

        Ok(Self { data, ptr: 0 })
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

    pub fn exec(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let instruction = self.next_instruction()?;
            self.exec_instruction(instruction)?;
        }
    }

    fn next_instruction(&mut self) -> Result<INSTRUCTION, Box<dyn Error>> {
        let instruction = match self.next_number()?.value() {
            0 => INSTRUCTION::HALT,
            6 => {
                let a = self.next_number()?;
                INSTRUCTION::JMP(a)
            },
            7 => {
                let (a, b) = self.next_two_numbers()?;
                INSTRUCTION::JT(a, b)
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
                self.ptr = a.value() as usize;
            },
            INSTRUCTION::JT(a, b) => {
                if a.value() != 0 {
                    self.ptr = b.value() as usize;
                }
            }
            INSTRUCTION::OUT(a) => {
                print!("{}", a.value() as u8 as char);
            },
            INSTRUCTION::NOOP => {},
            _ => {},
        }

        Ok(())
    }
}
