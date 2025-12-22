use std::error::Error;

pub struct Memory {
    registers: [u16; 8],
    stack: Vec<u16>,
    heap: [u16; 32768],
}

impl Memory {
    pub fn new() -> Self {
        Self { registers: [0; 8], stack: Vec::new(), heap: [0; 32768] }
    }

    pub fn read(&self, idx: u16) -> Result<u16, Box<dyn Error>> {
        if idx <= 32767 {
            return self.read_heap(idx);
        }

        if idx >= 32768 && idx <= 32775 {
            return self.read_register(idx);
        }

        return Err(format!("Invalid memory index: {}", idx).into());
    }

    pub fn write(&mut self, idx: u16, value: u16) -> Result<(), Box<dyn Error>> {
        if idx <= 32767 {
            return self.write_heap(idx, value);
        }

        if idx >= 32768 && idx <= 32775 {
            return self.write_register(idx, value);
        }

        return Err(format!("Invalid memory index: {}", idx).into());
    }

    pub fn read_register(&self, idx: u16) -> Result<u16, Box<dyn Error>> {
        Ok(self.registers[(idx % 32768) as usize])
    }

    pub fn write_register(&mut self, idx: u16, value: u16) -> Result<(), Box<dyn Error>> {
        self.registers[(idx % 32768) as usize] = value;
        Ok(())
    }

    pub fn read_heap(&self, idx: u16) -> Result<u16, Box<dyn Error>> {
        Ok(self.heap[idx as usize])
    }

    pub fn write_heap(&mut self, idx: u16, value: u16) -> Result<(), Box<dyn Error>> {
        self.heap[idx as usize] = value;
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
}
