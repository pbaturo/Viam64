mod mos6502;

fn main() {
    let mut cpu = mos6502::Mos6502::new();
    mos6502::soft_reset(&mut cpu);
    println!("Viam64 - A Commodore 64 emulator!");
}
