// This file is the core of our emulator

// chip8 has a 64x32 bit monochrome display, public so the frontend has access
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096; // Typical RAM size (KB) for chip8 emulator
const NUM_REGS: usize = 16; // chip8 has 16 8-bit registers which are much faster to access when compared RAM called V registers
const STACK_SIZE: usize = 16; // Stack can hold 16 numbers. It is used to return to starting point
                              // after a subroutine ends
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200; // PC must start at 512th bit according to chip8 specifications.
                               // 0x200 represents 512 in hex.
const FONTSET_SIZE: usize = 80;

// This creates the pixel assignments for each row in binary to display sprites 0-9 and A-F
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

pub struct Emu {
    pc: u16,  // This is the program counter, it is a special register that keeps an index of the current instruction
    ram: [u8; RAM_SIZE], // Array of 8 bit digits representin RAM
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16, // I register: used to index into RAM
    sp: u16, // Stack Pointer: Keeps index of the top of the stack
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8, // Delay Timer: Counts down every clock cycle, and performs an action when it hits 0
    st: u8, // Sound Timer: Counts down every clock cycle, and emits audio when it hits 0. This is
            // the only way to emit audio on the Chip-8
}

// This is the constructor for the Emu struct, we will initialize everything to 0 by default

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        
        new_emu
    }

    // Resets the emulator's values back to zero
    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
    
    // Used to push value to stack and increment Stack Pointer
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    // Moves SP back to previous value and returns current value. [EMPTY STACK CAUSES PANIC]
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        // Execute
    }

    fn fetch(&mut self) {
        
    }
}


















