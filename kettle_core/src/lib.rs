const NUM_REGS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
pub const DISP_WIDTH: usize = 64;
pub const DISP_LEN: usize = 32;
const START_ADDR: u16 = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
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
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ];


pub struct emu{
    pc: u16,
    ram: [u8; RAM_SIZE],
    display: [bool; DISP_WIDTH * DISP_LEN],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    sp: u16,
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8, 
}

impl emu{
    pub fn new() -> Self{
    let mut emulator = Self {
        pc: START_ADDR,
        ram: [0; RAM_SIZE],
        display: [false; DISP_WIDTH * DISP_LEN],
        v_reg: [0; NUM_REGS],
        i_reg: 0,
        stack: [0; STACK_SIZE],
        sp: 0,
        keys: [false; NUM_KEYS],
        dt: 0,
        st: 0,       
    };
        emulator.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        emulator
    }

    pub fn reset(&mut self){
        self.pc = START_ADDR;
        self.ram =[0; RAM_SIZE];
        self.display =[false; DISP_WIDTH * DISP_LEN];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, val: u16){
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16{
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn tick(&mut self){
        //fetch
        let op = self.fetch();
        //decode
        //execute
        self.execute(op);
    }

    fn fetch(&mut self) -> u16{
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[(self.pc +1) as usize] as u16;
        let op = (high_byte << 8) | low_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timers(&mut self){
        if(self.dt > 0){
            self.dt -=1;
        }
        if(self.st > 0){
            if (self.st == 1){
                //beep to implement
            } 
            self.st -= 1;
        }
    }

    fn execute(&mut self, op: u16){
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = (op & 0x00F);
        match(d1,d2,d3,d4){
            //nop
            (0,0,0,0) => return,
            //clear
            (0,0,0xE,0) => {self.screen = [false; DISP_WIDTH * DISP_LEN]},
            //return 
            (0, 0, 0xE, 0xE) => {
                let return_addr = self.pop();
                self.pc = return_addr;
            },

            //jump 1NNN
            (1,_,_,_) => {
                let nnn  = op & 0x0FFF;
                pc = nnn;
            },
            
            //call subroutine 2NNN
            (2, _,_,_) => {
                let nnn = op & 0x0FFF;
                self.push(self.pc);
                pc =  nnn;
            },

            //skip if vx == 0xnn
            (3,_,_,_) => {
                let x = op & 0x0F00;
                let nn = (op & 0x00FF) as u8; 
                if (self.v_reg[v] == nn){
                    self.pc += 2;
                }   
            },

            //skip if vx != 0xnn
            (4,_,_,_) => {
                let x = op & 0x0F00;
                let nn = (op & 0x00FF) as u8; 
                if (self.v_reg[v] != nn){
                    self.pc += 2;
                }   
            },

            //skip if vx == vy
            (5,_,_,0) => {
                let x = d2 as usize;
                let y = d3 as usize; 
                if (self.v_reg[v] == self.v_reg[y]){
                    self.pc += 2;
                }   
            },

            //set vx = 0xnn
            (6,_,_,_) => {
                let x = d2 as usize;
                let nn = (op & 0x00FF) as u8; 
                self.v_reg[x] = xx;
            },

            // vx += 0xnn
            (7,_,_,_) => {
                let x = d2 as usize;
                let nn = (op & 0x00FF) as u8; 
                self.v_reg[x] += self.v_reg[x].wrapping_add(nn);
            },

            (8,_,_,0) => {
                let x = d2 as usize;
                let y = d3 as usize; 
                self.v_reg[x] = self.v_reg[y];
            },

            
            (8,_,_,1) => {
                let x = d2 as usize;
                let y = d3 as usize; 
                self.v_reg[x] |= self.v_reg[y];
            },

            (8,_,_,2) => {
                let x = d2 as usize;
                let y = d3 as usize; 
                self.v_reg[x] &= self.v_reg[y];
            },

            (8,_,_,3) => {
                let x = d2 as usize;
                let y = d3 as usize; 
                self.v_reg[x] ^= self.v_reg[y];
            },

            // vx + vy, handles overflow (exceeds 255)
            // set vf to 1 if thers an overflow 
            (8,_,_,4) => {
                let x = d2 as usize;
                let y = d3 as usize; 
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            (8,_,_,5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            (8,_,_,6) => {
                let x = digit2 as usize;
                let dropped = (self.v_reg[x] & 1);
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = dropped;   
            },

            (8,_,_,7) => {
                let x = digit2 as usize;
                let y: usize = digit3 as usize;
                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            (8,_,_,0xE) => {
                let x = digit2 as usize;
                let dropped = ((self.v_reg[x] >> 7) & 1);
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = dropped;   
            },

            (9,_,_,0) => {
                let x = digit2 as usize;
                let y: usize = digit3 as usize;
                if self.v_reg[x] == self.v_ref[y]{
                    self.pc += 2;
                }
 
            },


            (_,_,_,_) => unimplemented!("Unimplemented opcode: {}", op),
        }
        
    } 

}