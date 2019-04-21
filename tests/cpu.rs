use chip8::Cpu;

#[test]
fn test_loading_bytes_from_vector() {
    let data = vec![0x1, 0x2, 0x3, 0x4];
    let mut cpu = Cpu::new();
    let mut memory = vec![0; 0x200];
    let mut program = vec![0; 3896];
    for (index, &byte) in data.iter().enumerate() {
        program[index] = byte;
    }
    memory.append(&mut program);
    cpu.load_program(data);
    for i in 0..4096 {
        assert_eq!(memory[i], cpu.memory[i])
    }
}
