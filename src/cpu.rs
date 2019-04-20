pub struct Cpu {
  pub opcode: u16,
  pub v: [u8; 16],
  pub i: u16,
  pub sound_timer: u8,
  pub delay_timer: u8,
  pub pc: u16,
  pub sp: u8,
  pub memory: [u8; 4096],
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      opcode: 0,
      v: [0; 16],
      i: 0x200,
      sound_timer: 0,
      delay_timer: 0,
      pc: 0x200,
      sp: 0,
      memory: [0; 4096],
    }
  }
}
