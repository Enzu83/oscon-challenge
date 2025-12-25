use std::error::Error;
use crate::num::{NumType, kind_of};

pub struct Memory {
    heap: [u16; 32768],
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            heap: [0; 32768],
            registers: [0; 8],
            stack: Vec::new(),
        }
    }

    pub fn load(&mut self, data: &[u16]) -> Result<(), Box<dyn Error>> {
        if data.len() > self.heap.len() {
            return Err(format!("Can't load data into memory, data too large: {}", data.len()).into());
        }

        self.heap[..data.len()].copy_from_slice(&data);
        Ok(())
    }

    pub fn value(&self, address: u16) -> Result<u16, Box<dyn Error>> {
        match kind_of(address) {
            NumType::LITERAL => Ok(address),
            NumType::REGISTER => self.value(self.registers[(address % 32768) as usize]),
            NumType::INVALID => {
                Err(format!("Attempt to read at invalid memory address: {}", address).into())
            }
        }
    }

    pub fn read(&self, address: u16) -> Result<u16, Box<dyn Error>> {
        match kind_of(address) {
            NumType::LITERAL => Ok(self.heap[address as usize]),
            NumType::REGISTER => self.read(self.registers[(address % 32768) as usize]),
            NumType::INVALID => {
                Err(format!("Attempt to read at invalid memory address: {}", address).into())
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u16) -> Result<(), Box<dyn Error>> {
        match kind_of(address) {
            NumType::LITERAL => Ok(self.heap[address as usize] = value),
            NumType::REGISTER => Ok(self.registers[(address % 32768) as usize] = value),
            NumType::INVALID => Err(format!(
                "Attempt to write {} at invalid memory address: {}",
                value, address
            )
            .into()),
        }
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
}
