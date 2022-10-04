struct CPU {
    registers: [u8; 16],
    pc: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    sp: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.pc;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn call(&mut self, addr: u16) {
        let sp = self.sp;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("stack overflow");
        }

        stack[sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    fn ret (&mut self) {
        if self.sp == 0 {
            panic!("stack underflow");
        }

        self.sp -= 1;
        let addr = self.stack[self.sp];
        self.pc = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
/*
 * PRIFRI, 2022.10.04:
 * - 마지막 register를 carry flag 사용.
 * - carry flag set. 여기선 별사용은 안함.
 */
        if overflow_detected {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >>  8) as u8;
            let y = ((opcode & 0x00f0) >>  4) as u8;
            let d = ((opcode & 0x000f) >>  0) as u8;
            let nnn = opcode & 0xfff;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0, 0, 0xe, 0xe) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode)
            }
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        pc: 0,
        stack: [0; 16],
        sp: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
/*
 * PRIFRI, 2022.10.04:
 * - 0x100 함수 call
 */
    mem[0x000] = 0x21; mem[0x001] = 0x00;
/*
 * PRIFRI, 2022.10.04:
 * - 0x100 함수 call
 */
    mem[0x002] = 0x21; mem[0x003] = 0x00;
/*
 * PRIFRI, 2022.10.04:
 * - 중지
 */
    mem[0x003] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xee;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
