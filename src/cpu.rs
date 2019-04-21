use std::process;

pub struct Cpu {
  opcode: u16,
  //v: [u8; 16],
  //i: u16,
  //sound_timer: u8,
  //delay_timer: u8,
  pc: usize,
  //sp: usize,
  memory: [u8; 4096],
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
      //sp: 0,
      memory: [0; 4096],
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
      0x1000 => self.opcode_jp_addr(),
      _ => self.opcode_unimplemented(),
    }
  }

  fn inc_pc(&mut self) {
    self.pc += 2;
  }

  fn opcode_unimplemented(&self) {
    println!("Error: opcode 0x{:x} is not implemented", self.opcode);
    self.exit()
  }

  //1NNN	Jump to address NNN
  fn opcode_jp_addr(&mut self) {
    self.pc = (self.opcode & 0x0fff) as usize;
    println!("pc: {:x?}", self.pc);
  }

  fn exit(&self) {
    println!("The emulator is exiting");
    process::exit(0);
  }
}
