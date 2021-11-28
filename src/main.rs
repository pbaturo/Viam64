mod mos6502;

fn main() {
    let mut cpu = mos6502::Mos6502::new();
    let mut ram = mos6502::Ram::new();
    mos6502::reset(&mut cpu);
    println!("Viam64 - A Commodore 64 emulator!");
}
