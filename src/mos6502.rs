pub const MEM_SIZE_64K : usize = 1024*64;

pub struct Ram {
    mem : [u8; MEM_SIZE_64K]
}

impl Ram {
    pub fn new() -> Self {
        Self {
            mem : [0; MEM_SIZE_64K]
        }
    }
}

pub struct Mos6502 {
    a_reg : u8,
    x_reg : u8,
    y_reg : u8,
    sp_reg : u16,
    pc_reg : u16,
    ps_reg : u8,
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
        }
    }      
}

pub fn reset(cpu : &mut Mos6502) {
    //not accurate however good enough for now
    cpu.a_reg = 0;
    cpu.x_reg = 0;
    cpu.y_reg = 0;
    cpu.sp_reg = 0;
    cpu.pc_reg = 0;
    cpu.ps_reg = 0;
}      

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn should_reset_function_set_registers_values_to_default() {
        //Arrange
        let mut cpu = Mos6502::new();
        cpu.a_reg = 3;
        cpu.x_reg = 24;
        cpu.y_reg = 222;
        cpu.sp_reg = 1;
        cpu.pc_reg = 45;
        cpu.ps_reg = 124;
        //Act
        reset(&mut cpu);
        //Assert
        assert_eq!(cpu.a_reg, 0);
        assert_eq!(cpu.x_reg, 0);
        assert_eq!(cpu.y_reg, 0);
        assert_eq!(cpu.sp_reg, 0);
        assert_eq!(cpu.pc_reg, 0);
        assert_eq!(cpu.ps_reg, 0);
    }
}
