use std::error::Error;

pub enum NumType {
    LITERAL,
    REGISTER,
    INVALID,
}

pub fn from_hex_slice(raw: &[u8]) -> Result<Vec<u16>, Box<dyn Error>> {
    if raw.len() % 2 != 0 {
        return Err(format!(
            "Size of hex slice should a multiple of 2. Got {}.",
            raw.len()
        )
        .into());
    }

    let mut num_slice = Vec::with_capacity(raw.len() / 2);

    for i in 0..raw.len() / 2 {
        let num = from_hex(&raw[2 * i..2 * i + 2])?;
        num_slice.push(num);
    }

    Ok(num_slice)
}

pub fn from_hex(raw: &[u8]) -> Result<u16, Box<dyn Error>> {
    if raw.len() != 2 {
        return Err(format!("Size of hex slice should be 2. Got {}.", raw.len()).into());
    }

    let hex = format!("{:02X}{:02X}", raw[1], raw[0]);
    let value = u16::from_str_radix(&hex, 16)?;

    Ok(value)
}

pub fn kind_of(value: u16) -> NumType {
    if value <= 32767 {
        return NumType::LITERAL;
    }

    if value >= 32768 && value <= 32775 {
        return NumType::REGISTER;
    }

    return NumType::INVALID;
}
