pub struct Clock {
    t_cycles: u32,
    m_cycles: u32,
}

// Master Clock Speed => 4.194304 MHz
// System Clock speed => MSP * (1/4)
// Every four t-states (system clock ticks) is one machine cycle
impl Clock {
    pub fn new() -> Self {
        Self {
            // System Clock Ticks
            t_cycles: 0,
            // Machine Cycles
            m_cycles: 0,
        }
    }

    pub fn cycle_clock(&mut self, cycles: u32) {
        if cycles < 1 ||cycles > 3 {
            panic!("Cycling clock should only be done in cycles of 1, 2, or 3 machine cycles");
        }

        self.t_cycles += cycles * 4;
        self.m_cycles += cycles;
    }

    pub fn reset_clock(&mut self) {
        // Just for debugging purposes
        println!("Resetting Clock!\nCurrent system clock ticks: {}\nCurrent Machine Cycles: {}",
            self.t_cycles, self.m_cycles);

        self.t_cycles = 0;
        self.m_cycles = 0;
    }
}