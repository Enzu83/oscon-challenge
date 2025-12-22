use std::error::Error;

#[derive(Clone)]
pub struct Number {
    value: u16,
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

        Ok(Self { value })
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn assert_literal(&self) -> Result<(), Box<dyn Error>> {
        if self.value > 32767 {
            return Err(format!("{} is not a literal value", self.value).into());
        }

        Ok(())
    }

    pub fn assert_register(&self) -> Result<(), Box<dyn Error>> {
        if self.value <= 32767 || self.value > 32775 {
            return Err(format!("{} is not a register", self.value).into());
        }
        
        Ok(())
    }

    pub fn assert_valid(&self) -> Result<(), Box<dyn Error>> {
        if self.value > 32775 {
            return Err(format!("{} is not valid", self.value).into());
        }
        
        Ok(())
    }
}