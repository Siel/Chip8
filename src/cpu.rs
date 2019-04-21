use std::process;

pub struct Cpu {
  opcode: u16,
  v: [u8; 16],
  //i: u16,
  //sound_timer: u8,
  //delay_timer: u8,
  pc: usize,
  sp: usize,
  memory: [u8; 4096],
  stack: [u16; 16],
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      opcode: 0,
      v: [0; 16],
      //i: 0x200,
      //sound_timer: 0,
      //delay_timer: 0,
      pc: 0x200,
      sp: 0,
      memory: [0; 4096],
      stack: [0; 16],
    }
  }

  pub fn load_program(&mut self, program: Vec<u8>) {
    let mut data = vec![0; 0x200]; //reserved portion of memory
    for byte in program {
      data.push(byte); //Injecting the program into the data vec
    }
    for (index, &byte) in data.iter().enumerate() {
      self.memory[index] = byte
    }
  }

  pub fn next_cycle(&mut self) {
    self.fetch_opcode();
    self.execute_opcode();
    //self.inc_pc();
  }

  fn fetch_opcode(&mut self) {
    self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    println!(
      "Fetching opcode at position 0x{:x}: 0x{:x}",
      self.pc, self.opcode
    )
  }

  fn execute_opcode(&mut self) {
    match self.opcode & 0xf000 {
      0x1000 => self.op_jp_addr(),
      0x2000 => self.op_call_addr(),
      0x3000 => self.op_se(),
      _ => self.op_unimplemented(),
    }
  }

  fn inc_pc(&mut self) {
    self.pc += 2;
  }

  fn op_unimplemented(&self) {
    println!("Error: opcode 0x{:x} is not implemented", self.opcode);
    self.exit()
  }

  //1NNN	Jump to address NNN
  fn op_jp_addr(&mut self) {
    self.pc = (self.opcode & 0x0fff) as usize;
    println!("pc: {:x?}", self.pc);
  }

  //2nnn - CALL addr
  //Call subroutine at nnn.
  //The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
  fn op_call_addr(&mut self) {
    self.stack[self.sp] = self.pc as u16;
    self.sp += 1;
    self.pc = (self.opcode & 0x0fff) as usize;
  }

  //3xkk - SE Vx, byte
  //Skip next instruction if Vx = kk.
  //The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
  fn op_se(&mut self) {
    if self.v[((self.opcode & 0x0f00) >> 8) as usize] == (self.opcode & 0x00ff) as u8 {
      self.inc_pc();
    }
    self.inc_pc();
  }

  fn exit(&self) {
    println!("The emulator is exiting");
    panic!("");
    //process::exit(0);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_loading_bytes_from_vector() {
    let data = vec![0x1, 0x2, 0x3, 0x4];
    let mut cpu = Cpu::new();
    let mut memory = vec![0; 0x200];
    let mut program = vec![0; 0xdff];
    for (index, &byte) in data.iter().enumerate() {
      program[index] = byte;
    }
    memory.append(&mut program);
    cpu.load_program(data);
    for i in 0..0xfff {
      assert_eq!(memory[i], cpu.memory[i])
    }
  }

  #[test]
  fn test_fetch_opcode() {
    let mut cpu = Cpu::new();
    cpu.load_program(vec![1, 1]);
    cpu.fetch_opcode();
    assert_eq!(0x0101, cpu.opcode);
  }

  #[test]
  fn test_op_jp_addr() {
    let mut cpu = Cpu::new();
    cpu.load_program(vec![0x13, 0x86]);
    cpu.next_cycle();
    assert_eq!(cpu.pc, 0x386);
  }

  #[test]
  fn test_op_call_addr() {
    let mut cpu = Cpu::new();
    cpu.load_program(vec![0x22, 0x04, 0x00, 0x00, 0x13, 0x86]);
    cpu.next_cycle();
    cpu.next_cycle();
    assert_eq!(cpu.pc, 0x386);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[0], 0x200);
  }

  #[test]
  fn test_op_se() {
    let mut cpu = Cpu::new();
    cpu.v[2] = 0x04;
    cpu.load_program(vec![0x32, 0x04, 0x00, 0x00, 0x13, 0x86]);
    cpu.next_cycle();
    cpu.next_cycle();
    assert_eq!(cpu.pc, 0x386);
  }

}
