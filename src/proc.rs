pub struct Process {
    ptr: usize,
    running: bool,
}

impl Process {
    pub fn new() -> Self {
        Self { ptr: 0, running: false }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn address(&self) -> u16 {
        self.ptr as u16
    }

    pub fn jump_to(&mut self, address: u16) {
        self.ptr = address as usize;
    }

    pub fn jump_to_next(&mut self) {
        self.ptr += 1;
    }
}