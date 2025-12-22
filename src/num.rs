use std::error::Error;

use crate::mem::Memory;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumType {
    LITERAL,
    REGISTER,
}

#[derive(Debug, Clone)]
pub struct Number {
    value: u16,
    kind: NumType,
}

impl Number {
    pub fn from_hex_slice(raw: &[u8]) -> Result<Vec<Self>, Box<dyn Error>> {
        if raw.len() % 2 != 0 {
            return Err(format!("Size of hex slice should a multiple of 2. Got {}.", raw.len()).into());
        }

        let mut num_slice = Vec::with_capacity(raw.len() / 2);

        for i in 0..raw.len() / 2 {
            let num = Number::from_hex(&raw[2*i..2*i+2])?;
            num_slice.push(num);
        }

        Ok(num_slice)
    }

    pub fn from_hex(raw: &[u8]) -> Result<Self, Box<dyn Error>> {
        if raw.len() != 2 {
            return Err(format!("Size of hex slice should be 2. Got {}.", raw.len()).into());
        }

        let hex = format!("{:02X}{:02X}", raw[1], raw[0]);
        let value = u16::from_str_radix(&hex, 16)?;        
        let kind = Number::kind_of(value)?;

        Ok(Self { value, kind })
    }

    pub fn kind_of(value: u16) -> Result<NumType, Box<dyn Error>> {
        if value <= 32767 {
            return Ok(NumType::LITERAL);
        }

        if value >= 32768 && value <= 32775 {
            return Ok(NumType::REGISTER);
        }

        return Err(format!("Value {} is invalid", value).into());
    }

    pub fn raw_value(&self) -> u16 {
        self.value
    }

    pub fn value(&self, mem: &Memory) -> u16 {
        match self.kind {
            NumType::LITERAL => self.raw_value(),
            NumType::REGISTER => mem.read_register(self.raw_value() as usize).unwrap(),
        }
    }

    pub fn kind(&self) -> &NumType {
        &self.kind
    }

    pub fn assert_literal(&self) -> Result<(), Box<dyn Error>> {
        if *self.kind() == NumType::LITERAL {
            return Ok(());
        }
        return Err(format!("{} is not a literal value", self.value).into());
    }

    pub fn assert_register(&self) -> Result<(), Box<dyn Error>> {
        if *self.kind() == NumType::REGISTER {
            return Ok(());
        }
        return Err(format!("{} is not a register", self.value).into());
    }
}