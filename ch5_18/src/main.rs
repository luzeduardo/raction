struct CPU {
    current_operation: u16, // opcodes are u16
    registers: [u8; 2], // only tw registers needed for addition
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
            let opcode = self.read_opcode();
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                // dipatches execution to hardware circuit to be performed
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        // }
    }

    fn add_xy(&mut self, x: u8, y:u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0, // initialize with no op
        registers: [0; 2],
    };

    // 8 means the operation involves two registers
    // 0 maps to register[0]
    // 1 maps to register[1]
    // 4 indicates addition
    cpu.current_operation = 0x8014;
    // registers can only hold u8 values
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("5 + 10 = {}", cpu.registers[0]);
}
