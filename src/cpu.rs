use std::process;

pub struct Cpu {
  opcode: u16,
  //v: [u8; 16],
  //i: u16,
  //sound_timer: u8,
  //delay_timer: u8,
  pc: usize,
  sp: usize,
  pub memory: [u8; 4096],
  stack: [u16; 16],
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      opcode: 0,
      //v: [0; 16],
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

  fn exit(&self) {
    println!("The emulator is exiting");
    process::exit(0);
  }
}
