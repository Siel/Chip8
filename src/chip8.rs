mod cpu;
mod graphics;
use std::sync::mpsc;

pub struct Chip8 {
    cpu: cpu::Cpu,
    graphics: graphics::Graphics
}

impl Chip8 {
    pub fn new() -> Chip8 {
       
        Chip8{
            cpu : cpu::Cpu::new(),
            graphics : graphics::Graphics::new()
        }
    }

    pub fn start(&mut self){
        
        self.cpu.load_program(vec![
            0x12, 0x25, 0x53, 0x50, 0x41, 0x43, 0x45, 0x20, 0x49, 0x4e, 0x56, 0x41, 0x44, 0x45, 0x52,
            0x53, 0x20, 0x30, 0x2e, 0x39, 0x31, 0x20, 0x42, 0x79, 0x20, 0x44, 0x61, 0x76, 0x69, 0x64,
            0x20, 0x57, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x60, 0x00, 0x61, 0x00, 0x62, 0x08, 0xa3, 0xdd,
            0xd0, 0x18, 0x71, 0x08, 0xf2, 0x1e, 0x31, 0x20, 0x12, 0x2d, 0x70, 0x08, 0x61, 0x00, 0x30,
            0x40, 0x12, 0x2d, 0x69, 0x05, 0x6c, 0x15, 0x6e, 0x00, 0x23, 0x91, 0x60, 0x0a, 0xf0, 0x15,
            0xf0, 0x07, 0x30, 0x00, 0x12, 0x4b, 0x23, 0x91, 0x7e, 0x01, 0x12, 0x45, 0x66, 0x00, 0x68,
            0x1c, 0x69, 0x00, 0x6a, 0x04, 0x6b, 0x0a, 0x6c, 0x04, 0x6d, 0x3c, 0x6e, 0x0f, 0x00, 0xe0,
            0x23, 0x75, 0x23, 0x51, 0xfd, 0x15, 0x60, 0x04, 0xe0, 0x9e, 0x12, 0x7d, 0x23, 0x75, 0x38,
            0x00, 0x78, 0xff, 0x23, 0x75, 0x60, 0x06, 0xe0, 0x9e, 0x12, 0x8b, 0x23, 0x75, 0x38, 0x39,
            0x78, 0x01, 0x23, 0x75, 0x36, 0x00, 0x12, 0x9f, 0x60, 0x05, 0xe0, 0x9e, 0x12, 0xe9, 0x66,
            0x01, 0x65, 0x1b, 0x84, 0x80, 0xa3, 0xd9, 0xd4, 0x51, 0xa3, 0xd9, 0xd4, 0x51, 0x75, 0xff,
            0x35, 0xff, 0x12, 0xad, 0x66, 0x00, 0x12, 0xe9, 0xd4, 0x51, 0x3f, 0x01, 0x12, 0xe9, 0xd4,
            0x51, 0x66, 0x00, 0x83, 0x40, 0x73, 0x03, 0x83, 0xb5, 0x62, 0xf8, 0x83, 0x22, 0x62, 0x08,
            0x33, 0x00, 0x12, 0xc9, 0x23, 0x7d, 0x82, 0x06, 0x43, 0x08, 0x12, 0xd3, 0x33, 0x10, 0x12,
            0xd5, 0x23, 0x7d, 0x82, 0x06, 0x33, 0x18, 0x12, 0xdd, 0x23, 0x7d, 0x82, 0x06, 0x43, 0x20,
            0x12, 0xe7, 0x33, 0x28, 0x12, 0xe9, 0x23, 0x7d, 0x3e, 0x00, 0x13, 0x07, 0x79, 0x06, 0x49,
            0x18, 0x69, 0x00, 0x6a, 0x04, 0x6b, 0x0a, 0x6c, 0x04, 0x7d, 0xf4, 0x6e, 0x0f, 0x00, 0xe0,
            0x23, 0x51, 0x23, 0x75, 0xfd, 0x15, 0x12, 0x6f, 0xf7, 0x07, 0x37, 0x00, 0x12, 0x6f, 0xfd,
            0x15, 0x23, 0x51, 0x8b, 0xa4, 0x3b, 0x12, 0x13, 0x1b, 0x7c, 0x02, 0x6a, 0xfc, 0x3b, 0x02,
            0x13, 0x23, 0x7c, 0x02, 0x6a, 0x04, 0x23, 0x51, 0x3c, 0x18, 0x12, 0x6f, 0x00, 0xe0, 0xa4,
            0xdd, 0x60, 0x14, 0x61, 0x08, 0x62, 0x0f, 0xd0, 0x1f, 0x70, 0x08, 0xf2, 0x1e, 0x30, 0x2c,
            0x13, 0x33, 0x60, 0xff, 0xf0, 0x15, 0xf0, 0x07, 0x30, 0x00, 0x13, 0x41, 0xf0, 0x0a, 0x00,
            0xe0, 0xa7, 0x06, 0xfe, 0x65, 0x12, 0x25, 0xa3, 0xc1, 0xf9, 0x1e, 0x61, 0x08, 0x23, 0x69,
            0x81, 0x06, 0x23, 0x69, 0x81, 0x06, 0x23, 0x69, 0x81, 0x06, 0x23, 0x69, 0x7b, 0xd0, 0x00,
            0xee, 0x80, 0xe0, 0x80, 0x12, 0x30, 0x00, 0xdb, 0xc6, 0x7b, 0x0c, 0x00, 0xee, 0xa3, 0xd9,
            0x60, 0x1c, 0xd8, 0x04, 0x00, 0xee, 0x23, 0x51, 0x8e, 0x23, 0x23, 0x51, 0x60, 0x05, 0xf0,
            0x18, 0xf0, 0x15, 0xf0, 0x07, 0x30, 0x00, 0x13, 0x89, 0x00, 0xee, 0x6a, 0x00, 0x8d, 0xe0,
            0x6b, 0x04, 0xe9, 0xa1, 0x12, 0x57, 0xa6, 0x0c, 0xfd, 0x1e, 0xf0, 0x65, 0x30, 0xff, 0x13,
            0xaf, 0x6a, 0x00, 0x6b, 0x04, 0x6d, 0x01, 0x6e, 0x01, 0x13, 0x97, 0xa5, 0x0a, 0xf0, 0x1e,
            0xdb, 0xc6, 0x7b, 0x08, 0x7d, 0x01, 0x7a, 0x01, 0x3a, 0x07, 0x13, 0x97, 0x00, 0xee, 0x3c,
            0x7e, 0xff, 0xff, 0x99, 0x99, 0x7e, 0xff, 0xff, 0x24, 0x24, 0xe7, 0x7e, 0xff, 0x3c, 0x3c,
            0x7e, 0xdb, 0x81, 0x42, 0x3c, 0x7e, 0xff, 0xdb, 0x10, 0x38, 0x7c, 0xfe, 0x00, 0x00, 0x7f,
            0x00, 0x3f, 0x00, 0x7f, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x03, 0x03, 0x03, 0x03, 0x00,
            0x00, 0x3f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3f, 0x08, 0x08, 0xff, 0x00,
            0x00, 0xfe, 0x00, 0xfc, 0x00, 0xfe, 0x00, 0x00, 0x00, 0x7e, 0x42, 0x42, 0x62, 0x62, 0x62,
            0x62, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00,
            0xff, 0x00, 0x7d, 0x00, 0x41, 0x7d, 0x05, 0x7d, 0x7d, 0x00, 0x00, 0xc2, 0xc2, 0xc6, 0x44,
            0x6c, 0x28, 0x38, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
            0x00, 0x00, 0xff, 0x00, 0xf7, 0x10, 0x14, 0xf7, 0xf7, 0x04, 0x04, 0x00, 0x00, 0x7c, 0x44,
            0xfe, 0xc2, 0xc2, 0xc2, 0xc2, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xef, 0x20, 0x28, 0xe8, 0xe8, 0x2f, 0x2f, 0x00, 0x00,
            0xf9, 0x85, 0xc5, 0xc5, 0xc5, 0xc5, 0xf9, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xbe, 0x00, 0x20, 0x30, 0x20, 0xbe, 0xbe,
            0x00, 0x00, 0xf7, 0x04, 0xe7, 0x85, 0x85, 0x84, 0xf4, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x7f, 0x00, 0x3f, 0x00,
            0x7f, 0x00, 0x00, 0x00, 0xef, 0x28, 0xef, 0x00, 0xe0, 0x60, 0x6f, 0x00, 0x00, 0xff, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0xfe, 0x00,
            0xfc, 0x00, 0xfe, 0x00, 0x00, 0x00, 0xc0, 0x00, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0x00, 0x00,
            0xfc, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0xfc, 0x10, 0x10, 0xff, 0xf9, 0x81,
            0xb9, 0x8b, 0x9a, 0x9a, 0xfa, 0x00, 0xfa, 0x8a, 0x9a, 0x9a, 0x9b, 0x99, 0xf8, 0xe6, 0x25,
            0x25, 0xf4, 0x34, 0x34, 0x34, 0x00, 0x17, 0x14, 0x34, 0x37, 0x36, 0x26, 0xc7, 0xdf, 0x50,
            0x50, 0x5c, 0xd8, 0xd8, 0xdf, 0x00, 0xdf, 0x11, 0x1f, 0x12, 0x1b, 0x19, 0xd9, 0x7c, 0x44,
            0xfe, 0x86, 0x86, 0x86, 0xfc, 0x84, 0xfe, 0x82, 0x82, 0xfe, 0xfe, 0x80, 0xc0, 0xc0, 0xc0,
            0xfe, 0xfc, 0x82, 0xc2, 0xc2, 0xc2, 0xfc, 0xfe, 0x80, 0xf8, 0xc0, 0xc0, 0xfe, 0xfe, 0x80,
            0xf0, 0xc0, 0xc0, 0xc0, 0xfe, 0x80, 0xbe, 0x86, 0x86, 0xfe, 0x86, 0x86, 0xfe, 0x86, 0x86,
            0x86, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x18, 0x18, 0x18, 0x48, 0x48, 0x78, 0x9c, 0x90,
            0xb0, 0xc0, 0xb0, 0x9c, 0x80, 0x80, 0xc0, 0xc0, 0xc0, 0xfe, 0xee, 0x92, 0x92, 0x86, 0x86,
            0x86, 0xfe, 0x82, 0x86, 0x86, 0x86, 0x86, 0x7c, 0x82, 0x86, 0x86, 0x86, 0x7c, 0xfe, 0x82,
            0xfe, 0xc0, 0xc0, 0xc0, 0x7c, 0x82, 0xc2, 0xca, 0xc4, 0x7a, 0xfe, 0x86, 0xfe, 0x90, 0x9c,
            0x84, 0xfe, 0xc0, 0xfe, 0x02, 0x02, 0xfe, 0xfe, 0x10, 0x30, 0x30, 0x30, 0x30, 0x82, 0x82,
            0xc2, 0xc2, 0xc2, 0xfe, 0x82, 0x82, 0x82, 0xee, 0x38, 0x10, 0x86, 0x86, 0x96, 0x92, 0x92,
            0xee, 0x82, 0x44, 0x38, 0x38, 0x44, 0x82, 0x82, 0x82, 0xfe, 0x30, 0x30, 0x30, 0xfe, 0x02,
            0x1e, 0xf0, 0x80, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x06, 0x06, 0x00, 0x00, 0x00, 0x60, 0x60,
            0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x7c, 0xc6,
            0x0c, 0x18, 0x00, 0x18, 0x00, 0x00, 0xfe, 0xfe, 0x00, 0x00, 0xfe, 0x82, 0x86, 0x86, 0x86,
            0xfe, 0x08, 0x08, 0x08, 0x18, 0x18, 0x18, 0xfe, 0x02, 0xfe, 0xc0, 0xc0, 0xfe, 0xfe, 0x02,
            0x1e, 0x06, 0x06, 0xfe, 0x84, 0xc4, 0xc4, 0xfe, 0x04, 0x04, 0xfe, 0x80, 0xfe, 0x06, 0x06,
            0xfe, 0xc0, 0xc0, 0xc0, 0xfe, 0x82, 0xfe, 0xfe, 0x02, 0x02, 0x06, 0x06, 0x06, 0x7c, 0x44,
            0xfe, 0x86, 0x86, 0xfe, 0xfe, 0x82, 0xfe, 0x06, 0x06, 0x06, 0x44, 0xfe, 0x44, 0x44, 0xfe,
            0x44, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0x6c, 0x5a, 0x00, 0x0c, 0x18, 0xa8, 0x30,
            0x4e, 0x7e, 0x00, 0x12, 0x18, 0x66, 0x6c, 0xa8, 0x5a, 0x66, 0x54, 0x24, 0x66, 0x00, 0x48,
            0x48, 0x18, 0x12, 0xa8, 0x06, 0x90, 0xa8, 0x12, 0x00, 0x7e, 0x30, 0x12, 0xa8, 0x84, 0x30,
            0x4e, 0x72, 0x18, 0x66, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0x90, 0x54, 0x78, 0xa8, 0x48,
            0x78, 0x6c, 0x72, 0xa8, 0x12, 0x18, 0x6c, 0x72, 0x66, 0x54, 0x90, 0xa8, 0x72, 0x2a, 0x18,
            0xa8, 0x30, 0x4e, 0x7e, 0x00, 0x12, 0x18, 0x66, 0x6c, 0xa8, 0x72, 0x54, 0xa8, 0x5a, 0x66,
            0x18, 0x7e, 0x18, 0x4e, 0x72, 0xa8, 0x72, 0x2a, 0x18, 0x30, 0x66, 0xa8, 0x30, 0x4e, 0x7e,
            0x00, 0x6c, 0x30, 0x54, 0x4e, 0x9c, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0x48, 0x54,
            0x7e, 0x18, 0xa8, 0x90, 0x54, 0x78, 0x66, 0xa8, 0x6c, 0x2a, 0x30, 0x5a, 0xa8, 0x84, 0x30,
            0x72, 0x2a, 0xa8, 0xd8, 0xa8, 0x00, 0x4e, 0x12, 0xa8, 0xe4, 0xa2, 0xa8, 0x00, 0x4e, 0x12,
            0xa8, 0x6c, 0x2a, 0x54, 0x54, 0x72, 0xa8, 0x84, 0x30, 0x72, 0x2a, 0xa8, 0xde, 0x9c, 0xa8,
            0x72, 0x2a, 0x18, 0xa8, 0x0c, 0x54, 0x48, 0x5a, 0x78, 0x72, 0x18, 0x66, 0xa8, 0x66, 0x18,
            0x5a, 0x54, 0x66, 0x72, 0x6c, 0xa8, 0x72, 0x2a, 0x00, 0x72, 0xa8, 0x72, 0x2a, 0x18, 0xa8,
            0x30, 0x4e, 0x7e, 0x00, 0x12, 0x18, 0x66, 0x6c, 0xa8, 0x00, 0x66, 0x18, 0xa8, 0x30, 0x4e,
            0x0c, 0x66, 0x18, 0x00, 0x6c, 0x30, 0x4e, 0x24, 0xa8, 0x72, 0x2a, 0x18, 0x30, 0x66, 0xa8,
            0x1e, 0x54, 0x66, 0x0c, 0x18, 0x9c, 0xa8, 0x24, 0x54, 0x54, 0x12, 0xa8, 0x42, 0x78, 0x0c,
            0x3c, 0xa8, 0xae, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]); 
            
        self.graphics.start_graphics();

        for _ in 1..1000 {
            //self.graphics.incx();
        }

    }
}