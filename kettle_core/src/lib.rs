const NUM_REGS: usize = 16;
const RAM_SIZE: usize = 4069;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
pub const DISP_WIDTH: usize = 64;
pub const DISP_LEN: usize = 32;
const START_ADDR: u16 = 0x200;


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
    Self {
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
    }
}
}