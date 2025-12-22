use std::error::Error;

use crate::num::Number;

pub struct Memory {
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self { registers: [0; 8], stack: Vec::new() }
    }

    fn assert_valid_register_index(idx: usize) -> Result<(), Box<dyn Error>> {
        if idx >= 8 {
            return Err(format!("Register index {} out of bounds", idx).into());
        }

        Ok(())
    }

    pub fn read_register(&self, idx: usize) -> Result<u16, Box<dyn Error>> {
        Memory::assert_valid_register_index(idx % 32768)?;
        Ok(self.registers[idx % 32768])
    }

    pub fn write_register(&mut self, idx: usize, value: u16) -> Result<(), Box<dyn Error>> {
        Memory::assert_valid_register_index(idx % 32768)?;
        self.registers[idx % 32768] = value;
        Ok(())
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn pop_stack(&mut self) -> Result<u16, Box<dyn Error>> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(format!("Stack is empty, cannot pop values out").into()),
        }
    }

    pub fn value_of(&self, number: &Number) -> Result<u16, Box<dyn Error>> {
        if number.is_literal() {
            return Ok(number.value());
        }

        if number.is_valid() {
            return self.read_register(number.value() as usize);
        }

        Err(format!("Can't get value of {:?}", number).into())
    }
}