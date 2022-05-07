//! # CHIP8
//!
//! `CHIP8` is a chip-8 emulator which still on development progress.

#![allow(non_snake_case)]
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    /// Reads opcode.
    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.memory[self.position_in_memory] as u16;
        let op_byte2 = self.memory[self.position_in_memory + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    /// Machine cycle.
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            let kk = (opcode & 0x00FF) as u8;
            let addr = opcode & 0x0FFF;
            

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0) => self.cls(),
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jmp(addr),
                (0x2, _, _, _) => self.call(addr),
                (0x3, _, _, _) => self.se(x, kk),
                (0x4, _, _, _) => self.sne(x, kk),
                (0x5, _, _, _) => self.se(x, y),
                (0x6, _, _, _) => self.ld(x, kk),
                (0x7, _, _, _) => self.add(x, kk),
                (0x8, _, _, _) => match d {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => self.add_xy(x, y),
                    _ => todo!("opcode {:04x}", opcode),
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    /// op: 00E0 - Clear the display.
    fn cls(&self) {
        println!("CLEAR SCREEN (TODO)")
    }

    /// op: 00ee - Ret return from the current sub-routine.
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack overflow!");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    /// op: 1nnn - Jump to addr(nnn).
    fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    /// op: 2nnn - Call sub-routine at 'addr(nnn)'.
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    /// op: 3xkk - Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.position_in_memory += 2;
        }
    }

    /// op: 4xkk -Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.position_in_memory +=2;
        }
    }

    ///  op: 6xkk - LD sets the value 'kk' into register Vx.
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    /// op: 7xkk - Add sets the value 'kk' into register Vx.
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    /// op: 8xy1 - Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    /// op: 8xy2 - Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }
    
    /// op: 8xy3 - Performs a bitwise XOR on the values of Vx and Vy, then stores the result in Vx.
    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    /// op: 8xy4 - Performs Vx + Vy and stores the result in Vx.
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        //Don't know what to do here yet :(
        if overflow {
            todo!("add_xy overflow")
        } else {
            //todo!("add_xy overflow")
        }
    }

}

fn main() {
    //Creates our CPU
    let mut cpu = CPU{
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    //V0 = 5, V1 =10
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    //instructions for V0 + (V1 * 2) + (V1 * 2) = V0
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
    //Starts the whole process
    cpu.run();

    //Tests if our CPU has finished its task successfully or not
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
