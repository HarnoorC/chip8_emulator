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

    // Ticks to the next clock cycle.
    pub fn tick(&mut self) {
        // Fetch: Retrieves opcode to be executed
        let op = self.fetch();
        // Decode & Execute
        self.execute(op);
    }

    fn execute(&mut self, op: u16) {
        // Separate each hex digit
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // NOP: Moves to next opcode, sometimes necessary for timing or alignment
            (0, 0, 0, 0) => return,
            
            // CLS: Clear Screen
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },

            // RET: Return from subroutine
            // A subroutine is like a jump in assembly, we move the PC to a specificed address
            // and continue exectuion, expect a subroutine is expected to be completed.
            // We will store the address of the line we want to return to in the stack. The stack also
            // allows us to have nested subroutines because we can keep stacking the addresses.
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },

            // 1NNN - Jump
            // JMP NNN: Moves the PC to given address.
            (1, _, _, _) => {
                let nnn = op & 0xFFF; // Cuts down op code to last 3 hex digits
                self.pc = nnn;
            },

            // 2NNN - Call Subroutine
            // CALL NNN
            // Pushes current PC to stack and then jumps to given address (nnn)
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            },

            // 3XNN - Skip next if VX == NN
            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2; // Skips one opcode
                }
            },

            // 4XNN - Skip next if VX != NN
            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },

            // 5XY0 - Skip next if VX == VY
            // SKIP VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // 6XNN - VX == NN
            // VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },

            // 7XNN - VX += NN
            // VX = VX + NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn); // wrapping_add wraps back around to 0 after max is reached
            }

            // 8XY0 - VX = VY
            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }

            // 8XY1, 8XY2, 8XY3 - Bitwise Operations
            // 8XY1 - Bitwise OR
            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            }

            // 8XY2 - Bitwise OR
            // VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            }

            // 8XY3 - Bitwise OR
            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            }

            // Panics when an unimplemented opcode is run
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    // Retreives the next opcode to be executed.
    fn fetch(&mut self) -> u16 {
        // Gets left two hex values from first RAM index
        let higher_byte = self.ram[self.pc as usize] as u16;
        // Gets right two hex values from second RAM index
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        // Shifts left byte to the left hand side and bitwise or to include right side
        let op = (higher_byte << 8) | lower_byte;
        // Increments PC counter by 2
        self.pc += 2;
        op
    }

    // Implementing delay and sound timers. These are updated every frame rather than every cycle
    // so they need a separate function.
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP 
                // NOTE: (audio will not be implemented in this emulator but this is the format)
            }
            self.st -= 1;
        }
    }
}


















