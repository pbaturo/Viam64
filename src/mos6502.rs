pub const MEM_SIZE_64K : usize = 1024*64;

pub struct Mos6502 {
    a_reg : u8,
    x_reg : u8,
    y_reg : u8,
    sp_reg : u8,
    pc_reg : u8,
    ps_reg : u8,

    mem : [u8; MEM_SIZE_64K]
}    

impl Mos6502 {
    pub fn new() -> Self {
        Self {
            a_reg : 0,
            x_reg : 0,
            y_reg : 0,
            sp_reg : 0,
            pc_reg : 0,
            ps_reg : 0,
            mem : [0; MEM_SIZE_64K]
        }
    }
}

pub fn soft_reset(cpu : &mut Mos6502) {
    cpu.a_reg = 0;
    cpu.x_reg = 0;
    cpu.y_reg = 0;
    cpu.sp_reg = 0;
    cpu.pc_reg = 0;
    cpu.ps_reg = 0;
}