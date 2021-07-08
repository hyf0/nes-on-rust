pub struct CPU {
    pub register_a: u8, // Accumulator
    pub register_x: u8, // Index Register
    pub status: u8,
    // Processor Status NV_BDIZC
    //
    pub program_counter: u16,
}

mod ins {
    // instructions
    pub const LDA: u8 = 0xA9;
    pub const TAX: u8 = 0xAA;
    pub const INX: u8 = 0xE8;

    // interrupts
    pub const BRK: u8 = 0x00;
}

mod status_flag {

    // C	Carry Flag	Not affected
    // Z	Zero Flag	Set if A = 0
    pub const ZERO_ON: u8 = 0b0000_0010;
    pub const ZERO_OFF: u8 = 0b1111_1101;
    // I	Interrupt Disable	Not affected
    // D	Decimal Mode Flag	Not affected
    // B	Break Command	Not affected
    // V	Overflow Flag	Not affected
    // N	Negative Flag
    pub const NEGATIVE_ON: u8 = 0b1000_0000;
    pub const NEGATIVE_OFF: u8 = 0b0111_1111;
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    
    fn inx(&mut self) {
      use std::num::Wrapping;
      let result = Wrapping(self.register_x) + Wrapping(1);
      self.register_x = result.0;
      self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= status_flag::ZERO_ON;
        } else {
            self.status &= status_flag::ZERO_OFF;
        }

        if result & 0b1000_0000 != 0 {
            self.status |= status_flag::NEGATIVE_ON;
        } else {
            self.status &= status_flag::NEGATIVE_OFF;
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let ops_code = program[self.program_counter as usize];
            self.program_counter += 1;
            match ops_code {
                ins::LDA => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }
                ins::TAX => self.tax(),
                ins::INX => self.inx(),
                ins::BRK => return,
                _ => todo!(""),
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & status_flag::ZERO_ON == 0);
        assert!(cpu.status & status_flag::NEGATIVE_ON == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & status_flag::ZERO_ON == 0b0000_0010);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1010;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
  
        assert_eq!(cpu.register_x, 0b1100_0001);
    }
 
     #[test]
     fn test_inx_overflow() {
         let mut cpu = CPU::new();
         cpu.register_x = 0xff;
         cpu.interpret(vec![0xe8, 0xe8, 0x00]);
 
         assert_eq!(cpu.register_x, 1)
     }
}
