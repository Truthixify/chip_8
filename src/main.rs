use chip_8::CPU;


fn main() {
  let mut cpu = CPU::new();
  cpu.load_font_into_memory();

  cpu.registers[0] = 5;
  cpu.registers[1] = 10;

  // 
  cpu.memory[0x200] = 0x24; cpu.memory[0x201] = 0x00; 
  cpu.memory[0x202] = 0x24; cpu.memory[0x203] = 0x00;

  cpu.memory[0x400] = 0xD0; cpu.memory[0x401] = 0x05; 
  cpu.memory[0x400] = 0xD0; cpu.memory[0x401] = 0x1F; 
  // cpu.memory[0x402] = 0xD0; cpu.memory[0x403] = 0x15;
  // cpu.memory[0x404] = 0xD0; cpu.memory[0x405] = 0x15;

  cpu.memory[0x406] = 0x00; cpu.memory[0x407] = 0xEE;

  // cpu.run();
  for (i, d) in cpu.display.iter().enumerate() {
    if (d >> i) & 1 == 1 {
      print!("*")
    } else {
        print!(" ")
    }
  }

  // println!("{:?}", cpu.display);
    
}