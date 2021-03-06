//use std::process;
use std::sync::mpsc;
use std::time::{Duration, Instant};
const LEGACY: bool = false;

pub struct Cpu {
  opcode: u16,
  v: [u8; 16],
  i: u16,
  sound_timer: u8,
  delay_timer: u8,
  pc: usize,
  sp: usize,
  memory: [u8; 4096],
  stack: [u16; 16],
  vram: [[bool; 64]; 32],
  key_buffer: [bool; 16],
  tx: mpsc::Sender<[[bool; 64]; 32]>,
  update_vram: bool,
  time_at_last_timer_count: Instant,
}

impl Cpu {
  pub fn new(tx: mpsc::Sender<[[bool; 64]; 32]>) -> Cpu {
    Cpu {
      opcode: 0,
      v: [0; 16],
      i: 0x200,
      sound_timer: 0,
      delay_timer: 0,
      pc: 0x200,
      sp: 0,
      memory: [0; 4096],
      stack: [0; 16],
      vram: [[false; 64]; 32],
      key_buffer: [false; 16],
      tx: tx,
      update_vram: false,
      time_at_last_timer_count: Instant::now()
    }
  }


  pub fn load_program(&mut self, program: Vec<u8>) {
    let mut data = vec![0; 0x200]; //reserved portion of memory
    for i in 0..80 {
      data[i] = FONT_SPRITES[i];
    }
    for byte in program {
      data.push(byte); //Injecting the program into the data vec
    }
    for (index, &byte) in data.iter().enumerate() {
      self.memory[index] = byte
    }
  }

  pub fn next_cycle(&mut self) {
    self.update_vram = false;
    self.fetch_opcode();
    self.execute_opcode();
    if(self.update_vram){
      self.tx.send(self.vram);
    }
    self.count_timers();
    //self.inc_pc();
  }

  fn count_timers(&mut self) {
    if Instant::now() - self.time_at_last_timer_count >= Duration::from_millis(17) {
        self.time_at_last_timer_count = Instant::now();
        if self.sound_timer > 0 {
            self.sound_timer = self.sound_timer - 1;
        }
        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }
    }
}

  fn fetch_opcode(&mut self) {
    self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    println!(
      "Fetching opcode at position 0x{:x}: 0x{:x}",
      self.pc, self.opcode
    )
  }

  fn execute_opcode(&mut self) {
    let components = (
      ((self.opcode & 0xF000) >> 12) as u8,
      ((self.opcode & 0x0F00) >> 8) as u8,
      ((self.opcode & 0x00F0) >> 4) as u8,
      (self.opcode & 0x000F) as u8
    );
    let nnn = (self.opcode & 0x0FFF) as usize;
    let kk = (self.opcode & 0x00FF) as u8;
    let x = components.1 as usize;
    let y = components.2 as usize;
    let n = components.3 as usize;
    match components {
      (0x0, 0x0, 0xE, 0x0) => self.op_cls(),
      (0x0, 0x0, 0xE, 0xE) => self.op_ret(),
      (0x1, _, _, _) => self.op_jp_addr(nnn),
      (0x2, _, _, _) => self.op_call_addr(nnn),
      (0x3, _, _, _) => self.op_se(x, kk),
      (0x4, _, _, _) => self.op_sne(x, kk),
      (0x5, _, _, 0x0) => self.op_se_xy(x, y),
      (0x6, _, _, _) => self.op_ld_vx(x, kk),
      (0x7, _, _, _) => self.op_add_vx(x, kk),
      (0x8, _, _, 0x0) => self.op_ld_vx_vy(x, y),
      (0x8, _, _, 0x1) => self.op_or_vx_vy(x, y),
      (0x8, _, _, 0x2) => self.op_and_vx_vy(x, y),
      (0x8, _, _, 0x3) => self.op_xor_vy_vy(x, y),
      (0x8, _, _, 0x4) => self.op_add_vx_vy(x, y),
      (0x8, _, _, 0x5) => self.op_sub_vx_vy(x, y),
      (0x8, _, _, 0x6) => self.op_shr_vx_vy(x, y),
      (0x8, _, _, 0x7) => self.op_subn_vx_vy(x, y),
      (0x8, _, _, 0xE) => self.op_shl_vx_vy(x, y),
      (0x9, _, _, 0x0) => self.op_sne_vx_vy(x, y),
      (0xA, _, _, _) => self.op_ldi(nnn),
      (0xB, _, _, _) => self.op_jp_addr_v0(nnn),
      (0xC, _, _, _) => self.op_rnd_vx(x, kk),
      (0xD, _, _, _) => self.op_drw_vx_vy_n(x, y, n),
      (0xE, _, 0x9, 0xE) => self.op_skp_vx(x),
      (0xE, _, 0xA, 0x1) => self.op_sknp_vx(x),
      (0xF, _, 0x0, 0x7) => self.op_ld_vx_dt(x),
      (0xF, _, 0x0, 0xA) => self.op_ld_vx_k(x),
      (0xF, _, 0x1, 0x5) => self.op_ld_dt_vx(x),
      (0xF, _, 0x1, 0x8) => self.op_ld_st_vx(x),
      (0xF, _, 0x1, 0xE) => self.op_add_i_vx(x),
      (0xF, _, 0x2, 0x9) => self.op_ld_f_vx(x),
      (0xF, _, 0x3, 0x3) => self.op_ld_b_vx(x),
      (0xF, _, 0x5, 0x5) => self.op_ld_i_vx(x),
      (0xF, _, 0x6, 0x5) => self.op_ld_vx_i(x),
      (_, _, _, _) => self.op_unimplemented(),
    }
  }

  fn inc_pc(&mut self) {
    self.pc += 2;
  }

  fn op_unimplemented(&self) {
    println!("Error: opcode 0x{:x} is not implemented", self.opcode);
    self.exit()
  }

  // 00E0 - CLS -- Clear the display.
  fn op_cls(&mut self) {
    self.vram = [[false; 64]; 32];
    self.update_vram = true;
    self.inc_pc();
  }

  // 00EE - RET -- Return from a subroutine.
  // Sets program counter to address at the top of the stack, then subtracts 1 from
  // the stack pointer.
  fn op_ret(&mut self) {
    self.sp -= 1;
    self.pc = self.stack[self.sp] as usize;
    self.inc_pc();
  }

  //1NNN	Jump to address NNN
  fn op_jp_addr(&mut self, nnn: usize) {
    self.pc = nnn;
    //println!("pc: {:x?}", self.pc);
  }

  //2nnn - CALL addr
  //Call subroutine at nnn.
  //The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
  fn op_call_addr(&mut self, nnn: usize) {
    self.stack[self.sp] = self.pc as u16;
    self.sp += 1;
    self.pc = nnn;
  }

  //3xkk - SE Vx, byte
  //Skip next instruction if Vx = kk.
  //The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
  fn op_se(&mut self, x: usize, kk: u8) {
    if self.v[x] == kk {
      self.inc_pc();
    }
    self.inc_pc();
  }

  //4xkk - SNE Vx, byte
  //Skip next instruction if Vx != kk.
  //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
  fn op_sne(&mut self, x: usize, kk: u8) {
    if self.v[x] != kk {
      self.inc_pc();
    }
    self.inc_pc();
  }

  //5xy0 - SE Vx, Vy
  //Skip next instruction if Vx = Vy.
  //The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
  fn op_se_xy(&mut self, x: usize, y: usize) {
    if self.v[x] == self.v[y]{
      self.inc_pc();
    }
    self.inc_pc();
  }

  //6xkk - LD Vx, byte
  //Set Vx = kk.
  //The interpreter puts the value kk into register Vx.
  fn op_ld_vx(&mut self, x: usize, kk: u8) {
    self.v[x] = kk;
    self.inc_pc();
  }

  //7xkk - ADD Vx, byte
  //Set Vx = Vx + kk.
  //Adds the value kk to the value of register Vx, then stores the result in Vx.
  fn op_add_vx(&mut self, x: usize, kk: u8) {
    self.v[x] += kk;
    self.inc_pc();
  }

  //8xy0 - LD Vx, Vy
  //Set Vx = Vy.
  //Stores the value of register Vy in register Vx.
  fn op_ld_vx_vy(&mut self, x: usize, y: usize) {
    self.v[x] = self.v[y];
    self.inc_pc();
  }

  //8xy1 - OR Vx, Vy
  //Set Vx = Vx OR Vy.
  //Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
  fn op_or_vx_vy(&mut self, x: usize, y: usize) {
    self.v[x] |= self.v[y];
    self.inc_pc();
  }

  //8xy2 - AND Vx, Vy
  //Set Vx = Vx AND Vy.
  //Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
  fn op_and_vx_vy(&mut self, x: usize, y: usize) {
    self.v[x] &= self.v[y];
    self.inc_pc();
  }

  //8xy3 - XOR Vx, Vy
  //Set Vx = Vx XOR Vy.
  //Performs a bitwise exclusive OR on the values of Vx and Vy,
  fn op_xor_vy_vy(&mut self, x: usize, y: usize) {
    self.v[x] ^= self.v[y];
    self.inc_pc();
  }

  //8xy4 - ADD Vx, Vy
  //Set Vx = Vx + Vy, set VF = carry.
  //The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
  fn op_add_vx_vy(&mut self, x: usize, y: usize) {
    let sum = self.v[x] as u16 + self.v[y] as u16;
    self.v[0xf] = if sum > 0xff {1} else {0};
    self.v[x] = (sum & 0x00ff) as u8;
    self.inc_pc();
  }

  //8xy5 - SUB Vx, Vy #ERROR???
  //Set Vx = Vx - Vy, set VF = NOT borrow.
  //If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
  fn op_sub_vx_vy(&mut self, x: usize, y: usize) {
    if self.v[x] > self.v[y]
    {
      self.v[0xf] = 1;
    } else {
      self.v[0xf] = 0;
    }
    //wrapping prevents the execution to panic if the operation overflows
    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    self.inc_pc();
  }

  //8xy6 - SHR Vx Vy
  //Set Vx = Vy SHR 1.
  //LEGACY: If the least-significant bit of Vy is 1, then VF is set to 1, otherwise 0. Then Vy is divided by 2 and the results stored in Vx.
  //NEW: If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
  fn op_shr_vx_vy(&mut self, x: usize, y: usize) {
    if LEGACY {
      //vf = vy &0x0001
      self.v[0xf] = (self.v[y] & 0x0001) as u8;
      //vx = vy / 2
      self.v[x] = self.v[y] >> 1;
      self.inc_pc();
    } else {
      //vf = vx &0x0001
      self.v[0xf] = (self.v[x] & 0x0001) as u8;
      //vx = vx / 2
      self.v[x] = self.v[x] >> 1;
      self.inc_pc();
    }
  }

  //8xy7 - SUBN Vx, Vy
  //Set Vx = Vy - Vx, set VF = NOT borrow.
  //If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
  fn op_subn_vx_vy(&mut self, x: usize, y: usize) {
    if self.v[x] < self.v[y]{
      self.v[0xf] = 0;
    } else {
      self.v[0xf] = 1;
    }
    self.v[x] -= self.v[y];
    self.inc_pc();
  }

  //8xyE - SHL Vx {, Vy}
  //Set Vx = Vx SHL 1.
  //LEGACY: If the most-significant bit of Vy is 1, then VF is set to 1, otherwise to 0. Then Vy is multiplied by 2 and stored in Vx.
  //New: If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
  fn op_shl_vx_vy(&mut self, x: usize, y: usize) {
    if LEGACY {
      //vf = vy MSB
      self.v[0xf] = (self.v[y] >> 7) as u8;
      //vx = vy * 2
      self.v[x] = self.v[y] << 1;
      self.inc_pc();
    } else {
      //vf = vx MSB
      self.v[0xf] = (self.v[x] >> 7) as u8;
      //vx = vx * 2
      self.v[x] = self.v[x] << 1;
      self.inc_pc();
    }
  }

  //9xy0 - SNE Vx, Vy
  //Skip next instruction if Vx != Vy.
  //The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
  fn op_sne_vx_vy(&mut self, x: usize, y: usize) {
    if self.v[x] != self.v[y]{
      self.inc_pc();
    }
    self.inc_pc();
  }

  //Annn - LD I, addr
  //Set I = nnn.
  //The value of register I is set to nnn.
  fn op_ldi(&mut self, nnn: usize) {
    self.i = nnn as u16;
    self.inc_pc();
  }

  //Bnnn - JP V0, addr
  //Jump to location nnn + V0.
  //The program counter is set to nnn plus the value of V0.
  fn op_jp_addr_v0(&mut self, nnn: usize) {
    self.pc = nnn as usize + self.v[0x0] as usize;
  }

  //Cxkk - RND Vx, byte
  //Set Vx = random byte AND kk.
  //The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
  fn op_rnd_vx(&mut self, x: usize, kk: u8) {
    self.v[x] = rand::random::<u8>() & kk;
    self.inc_pc();
  }

  //Dxyn - DRW Vx, Vy, nibble
  //Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
  //The interpreter reads n bytes from memory, starting at the address stored in I.
  //These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
  //Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
  //VF is set to 1, otherwise it is set to 0.
  //If the sprite is positioned so part of it is outside the coordinates of the display,
  //it wraps around to the opposite side of the screen.
  fn op_drw_vx_vy_n(&mut self, x: usize, y: usize, n: usize) {
    self.v[0x0f] = 0;
    for byte in 0..n {
        let y = (self.v[y] as usize + byte) % 32;
        for bit in 0..8 {
            let x = (self.v[x] as usize + bit) % 64;
            let color = ((self.memory[(self.i as usize) + byte] >> (7 - bit)) & 1) == 1;
            self.v[0x0f] |= (color & self.vram[y][x]) as u8;
            self.vram[y][x] ^= color;

        }
    }
    self.update_vram = true;



    // let vx = self.v[x] as usize;
    // let vy = self.v[y] as usize;
    // let mut collision = 0;
    // let i = self.i as usize;
    // if vx > 63 || vy > 31 {
    //   return;
    // }
    // let sprite = &self.memory[i..i + n];
    // for y_offset in 0..n {
    //   if vy + y_offset > 31 {
    //     break;
    //   }
    //   let mut x_offset = vx + 8;
    //   if x_offset > 63 {
    //     x_offset = 63;
    //   }
    //   let screen_slice = &mut self.vram[vy + y_offset][vx..x_offset];
    //   let sprite_slice = sprite[y_offset];

    //   for pixel in 0..screen_slice.len() {
    //     let sprite_pixel = (sprite_slice & (0b1000_0000 >> pixel)) != 0; //casting to bool
    //     if screen_slice[pixel] && sprite_pixel {
    //       collision = 1
    //     }
    //     screen_slice[pixel] ^= sprite_pixel;
    //   }
    // }
    // self.v[0xf] = collision;
    self.inc_pc();
  }

  //Ex9E - SKP Vx
  //Skip next instruction if key with the value of Vx is pressed.
  //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
  fn op_skp_vx(&mut self, x: usize) {
    let key = self.v[x] as usize;
    if self.key_buffer[key] {
      self.inc_pc();
    }
    self.inc_pc();
  }

  //ExA1 - SKNP Vx
  //Skip next instruction if key with the value of Vx is not pressed.
  //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
  fn op_sknp_vx(&mut self, x: usize) {
    let key = self.v[x] as usize;
    if !self.key_buffer[key] {
      self.inc_pc();
    }
    self.inc_pc();
  }

  //Fx07 - LD Vx, DT
  //Set Vx = delay timer value.
  //The value of DT is placed into Vx.
  fn op_ld_vx_dt(&mut self, x: usize) {
    self.v[x] = self.delay_timer;
    self.inc_pc();
  }

  //Fx0A - LD Vx, K
  //Wait for a key press, store the value of the key in Vx.
  //All execution stops until a key is pressed, then the value of that key is stored in Vx.
  fn op_ld_vx_k(&mut self, x: usize) {
    let exit = false;
    for (i, pressed) in self.key_buffer.iter().enumerate() {
      if *pressed {
        self.v[x] = i as u8;
      }
    }
    if exit {
      self.inc_pc();
    }
  }

  //Fx15 - LD DT, Vx
  //Set delay timer = Vx.
  //DT is set equal to the value of Vx.
  fn op_ld_dt_vx(&mut self, x: usize) {
    self.delay_timer = self.v[x];
    self.inc_pc();
  }

  //Fx18 - LD ST, Vx
  //Set sound timer = Vx.
  //ST is set equal to the value of Vx.
  fn op_ld_st_vx(&mut self, x: usize) {
    self.sound_timer = self.v[x];
    self.inc_pc();
  }

  //Fx1E - ADD I, Vx
  //Set I = I + Vx.
  //The values of I and Vx are added, and the results are stored in I.
  fn op_add_i_vx(&mut self, x: usize) {
    self.i += self.v[x] as u16;
    self.inc_pc();
  }

  //Fx29 - LD F, Vx
  //Set I = location of sprite for digit Vx.
  //The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
  fn op_ld_f_vx(&mut self, x: usize) {
    //each sprite takes 5 bytes so 0x0000-> '0', 0x0005 -> '1', 0x000a -> '2' and so on.
    self.i = self.v[x] as u16 * 5;
    self.inc_pc();
  }

  //Fx33 - LD B, Vx
  //Store BCD representation of Vx in memory locations I, I+1, and I+2.
  //The interpreter takes the decimal value of Vx,
  //and places the hundreds digit in memory at location in I,
  //the tens digit at location I+1, and the ones digit at location I+2.
  fn op_ld_b_vx(&mut self, x: usize) {
    let vx = self.v[x];
    let i = self.i as usize;
    self.memory[i] = vx / 100;
    self.memory[i + 1] = (vx % 100) / 10;
    self.memory[i + 2] = (vx % 100) % 10;
    self.inc_pc();
  }

  //Fx55 - LD [I], Vx
  //Store registers V0 through Vx in memory starting at location I.
  //The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
  fn op_ld_i_vx(&mut self, x: usize) {
    let x = (x) as u16;
    for n in 0..x + 1 {
      self.memory[(self.i + n) as usize] = self.v[n as usize];
    }
    self.inc_pc();
  }

  //Fx65 - LD Vx, [I]
  //Read registers V0 through Vx from memory starting at location I.
  //The interpreter reads values from memory starting at location I into registers V0 through Vx.
  fn op_ld_vx_i(&mut self, x: usize) {
    let x = (x) as u16;
    for n in 0..x + 1 {
      self.v[n as usize] = self.memory[(self.i + n) as usize];
    }
    self.inc_pc();
  }

  fn exit(&self) {
    println!("The emulator is exiting");
    panic!("");
    //process::exit(0);
  }
}

static FONT_SPRITES: [u8; 80] = [
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
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// #[cfg(test)]
// mod test {
//   use super::*;
 

//   #[test]
//   fn test_loading_bytes_from_vector() {
//     let data = vec![0x1, 0x2, 0x3, 0x4];
//     let mut cpu = Cpu::new();
//     let mut memory = vec![0; 0x200];
//     let mut program = vec![0; 0xe00];
//     for (index, &byte) in data.iter().enumerate() {
//       program[index] = byte;
//     }
//     memory.append(&mut program);
//     cpu.load_program(data);
//     for i in 0..0x1000 {
//       assert_eq!(memory[i], cpu.memory[i])
//     }
//   }

//   #[test]
//   fn test_fetch_opcode() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![1, 1]);
//     cpu.fetch_opcode();
//     assert_eq!(0x0101, cpu.opcode);
//   }

//   #[test]
//   fn test_op_jp_addr() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x13, 0x86]);
//     cpu.next_cycle();
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_call_addr() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x22, 0x04, 0x00, 0x00, 0x13, 0x86]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.pc, 0x386);
//     assert_eq!(cpu.sp, 1);
//     assert_eq!(cpu.stack[0], 0x200);
//   }

//   #[test]
//   fn test_op_se() {
//     let mut cpu = Cpu::new();
//     cpu.v[2] = 0x04;
//     cpu.load_program(vec![0x32, 0x04, 0x00, 0x00, 0x13, 0x86]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_sne() {
//     let mut cpu = Cpu::new();
//     cpu.v[2] = 0x03;
//     cpu.load_program(vec![0x42, 0x04, 0x00, 0x00, 0x13, 0x86]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_se_xy() {
//     let mut cpu = Cpu::new();
//     cpu.v[2] = 0x03;
//     cpu.v[5] = 0x03;
//     cpu.load_program(vec![0x52, 0x50, 0x13, 0x47, 0x13, 0x86]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_no_op_se_xy() {
//     let mut cpu = Cpu::new();
//     cpu.v[2] = 0x03;
//     cpu.v[5] = 0x04;
//     cpu.load_program(vec![0x52, 0x50, 0x13, 0x47, 0x13, 0x86]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.pc, 0x347);
//   }

//   #[test]
//   fn test_op_ld_vx() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![
//       0x62, 0x03, 0x65, 0x03, 0x52, 0x50, 0x13, 0x47, 0x13, 0x86,
//     ]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[2], 0x03);
//     assert_eq!(cpu.v[5], 0x03);
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_add_vx() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![
//       0x62, 0x02, 0x65, 0x03, 0x72, 0x01, 0x52, 0x50, 0x13, 0x47, 0x13, 0x86,
//     ]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[2], 0x03);
//     assert_eq!(cpu.v[5], 0x03);
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_ld_vx_vy() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![
//       0x62, 0x03, 0x85, 0x20, 0x52, 0x50, 0x13, 0x47, 0x13, 0x86,
//     ]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[2], 0x03);
//     assert_eq!(cpu.v[5], 0x03);
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_or_vx_vy() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![
//       0x62, 0x03, 0x85, 0x21, 0x52, 0x50, 0x13, 0x47, 0x13, 0x86,
//     ]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[2], 0x03);
//     assert_eq!(cpu.v[5], 0x03);
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_and_vx_vy() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![
//       0x62, 0x03, 0x65, 0xff, 0x6a, 0x03, 0x85, 0xa2, 0x52, 0x50, 0x13, 0x47, 0x13, 0x86,
//     ]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[2], 0x03);
//     assert_eq!(cpu.v[5], 0x03);
//     assert_eq!(cpu.pc, 0x386);
//   }

//   #[test]
//   fn test_op_and_vx_vy_2() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x62, 0x03, 0x65, 0x05, 0x85, 0x22]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[5], 0x01);
//     assert_eq!(cpu.pc, 0x206);
//   }

//   #[test]
//   fn test_op_xor_vx_vy() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x62, 0xaa, 0x65, 0x3d, 0x85, 0x23]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[5], 0x97);
//   }

//   #[test]
//   fn test_op_add_vx_vy() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x62, 0xaa, 0x65, 0x01, 0x85, 0x24]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[0xf], 0);
//     assert_eq!(cpu.v[5], 0xab);
//   }

//   #[test]
//   fn test_op_add_vx_vy_with_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x62, 0xfa, 0x65, 0x10, 0x85, 0x24]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[5], 0x0a);
//     assert_eq!(cpu.v[0xf], 1);
//   }

//   #[test]
//   fn test_op_sub_vx_vy() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x62, 0x0a, 0x65, 0xaa, 0x85, 0x25]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[0xf], 0);
//     assert_eq!(cpu.v[5], 0xa0);
//   }

//   #[test]
//   fn test_op_sub_vx_vy_with_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.load_program(vec![0x62, 0xab, 0x65, 0xaa, 0x85, 0x25]);
//     cpu.next_cycle();
//     cpu.next_cycle();
//     cpu.next_cycle();
//     assert_eq!(cpu.v[0xf], 1);
//     assert_eq!(cpu.v[5], 0xff);
//   }
// }
