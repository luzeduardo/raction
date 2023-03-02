struct CPU {
    current_operation: u16, // opcodes are u16
    registers: [u8; 2], // only tw registers needed for addition
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
    cpu.registers[1] = 10
}
