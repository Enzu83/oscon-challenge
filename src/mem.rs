use std::{error::Error, fs::File, io::Write};

use crate::num::{kind_of, NumType};

pub struct Memory {
    heap: [u16; 32768],
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self { heap: [0; 32768], registers: [0; 8], stack: Vec::new() }
    }

    pub fn load(&mut self, data: &[u16]) -> Result<(), Box<dyn Error>> {
        if data.len() > self.heap.len() {
            return Err(format!("Can't load data into memory, too large: {}", data.len()).into());
        }

        self.heap[..data.len()].copy_from_slice(&data);
        Ok(())
    }

    pub fn value(&self, number: u16) -> Result<u16, Box<dyn Error>> {
        match kind_of(number) {
            NumType::LITERAL => Ok(number),
            NumType::REGISTER => {
                Ok(self.registers[(number % 32768) as usize])
            },
            NumType::INVALID => Err(format!("Invalid value: {}", number).into()),
        }
    }

    pub fn read(&self, address: u16) -> Result<u16, Box<dyn Error>> {
        match kind_of(address) {
            NumType::LITERAL => Ok(self.heap[address as usize]),
            NumType::REGISTER => {
                Ok(self.registers[(address % 32768) as usize])
            },
            NumType::INVALID => Err(format!("Attempt to read at invalid memory address: {}", address).into()),
        }
    }

    pub fn write(&mut self, address: u16, value: u16) -> Result<(), Box<dyn Error>> {
        match kind_of(address) {
            NumType::LITERAL => {
                self.heap[address as usize] = value;
                Ok(())
            },
            NumType::REGISTER => {
                self.registers[(address % 32768) as usize] = value;
                Ok(())
            },
            NumType::INVALID => Err(format!("Attempt to write {} at invalid memory address: {}", value, address).into()),
        }
    }

    pub fn read_register(&self, idx: u16) -> Result<u16, Box<dyn Error>> {
        Ok(self.registers[(idx % 32768) as usize])
    }

    pub fn write_register(&mut self, idx: u16, value: u16) -> Result<(), Box<dyn Error>> {
        self.registers[(idx % 32768) as usize] = value;
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

    pub fn registers(&self) -> &[u16; 8] {
        &self.registers
    }

    fn write_file(&self, path: &str, vals: &[u16]) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let mut vals_str = Vec::with_capacity(vals.len());
        for val in vals.iter() {
            vals_str.push(format!("{}", val));
        }
        file.write(vals_str.join("\n").as_bytes())?;

        Ok(())
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        std::fs::remove_dir_all("dump")?;
        std::fs::create_dir("dump")?;

        self.write_file("dump/registers.mem", &self.registers)?;
        self.write_file("dump/stack.mem", &self.stack)?;

        Ok(())
    }
}
