mod cpu;
use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_program(vec![0x13, 0xc5]);
    cpu.next_cycle();
    cpu.next_cycle();
}
