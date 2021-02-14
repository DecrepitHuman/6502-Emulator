pub type Byte = u8;
pub type Word = u16;

// Memory Structure
#[derive(Debug)]
pub struct Mem {
    MAX_MEM: u32,
    Data: [Byte; 1024 * 64]
}
// Implement create function
impl Mem {
    pub fn Init() -> Self {
        let mut arr = [0; 1024 * 64];
        for i in 0..arr.len() {
            arr[i] = 0;
        }

        Self {
            MAX_MEM: 1024 * 64,
            Data: arr
        }
    }
}

// CPU Structure
pub struct CPU {
    pub PC: Word, // Program Counter
    pub SP: Word, // Stack Pointer

    // Registers:
    pub A: Byte,
    pub X: Byte,
    pub Y: Byte,

    // Status Flags:
    pub C: Byte,
    pub Z: Byte,
    pub I: Byte,
    pub D: Byte,
    pub B: Byte,
    pub V: Byte,
    pub N: Byte
}
// Implement create function
impl CPU {
    pub fn Init() -> Self {
        Self {
            PC: 0xFFFC,
            SP: 0x0100,

            A: 0,
            X: 0,
            Y: 0,

            C: 0,
            Z: 0,
            I: 0,
            D: 0,
            B: 0,
            V: 0,
            N: 0,
        }
    }
    pub fn Execute(&self, Cycles: u32, mem: &Mem) {
        println!("{:?}", mem);
    }
}

pub fn main() {
    let cpu = CPU::Init(); // Start CPU
    let mem = Mem::Init(); // Allocate RAM

    cpu.Execute(2, &mem) // Execute ASM in allocated memory, not like it can understand it yet but for now it just shows we all the generous ram it has
}
