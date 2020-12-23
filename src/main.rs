
mod chip8;
use chip8::Chip8;

fn main() {
   let mut chip8 = Chip8::new();
   
    // for _ in 1..100 {
    //     cpu.next_cycle(); 
    // }
    chip8.start();
}
