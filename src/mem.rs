use std::{error::Error, fs::File, io::Write};

use crate::num::{kind_of, NumType};

pub struct Memory {
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self { registers: [0; 8], stack: Vec::new() }
    }

    pub fn read(&self, value: u16) -> Result<u16, Box<dyn Error>> {
        match kind_of(value) {
            NumType::LITERAL => Ok(value),
            NumType::REGISTER => self.read_register(value),
            NumType::INVALID => Err(format!("Invalid memory index: {}", value).into()),
        }
    }

    pub fn write(&mut self, idx: u16, value: u16) -> Result<(), Box<dyn Error>> {
        match kind_of(idx) {
            NumType::REGISTER => self.write_register(idx, value),
            _ => Err(format!("Invalid memory index: {}", value).into()),
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
