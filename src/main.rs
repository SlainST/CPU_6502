use std::{num::Wrapping, result};

use crate::AddressingMode::Accumulator;
fn main() {
    
    let _memory = Memory {
        data: [0x00u8; 65536],
    };
    let mut _cpu = CPU_6502 {
        address_bus: 0,
        data_bus: 0,
        accumulator: 0,
        index_registers: [0; 2],
        stack_pointer: 0,
        status_register: 0,
        program_counter: 0,
        memory: _memory,
    };
   
    
    clock();



    _cpu.decode_opcode(0xa9);

}
//not: Sayfa geçişi döngüsü. Her komut belli sayıda saat döngüsü sürer (senin cost değişkenin). Çip AbsoluteX’te X’i önce yalnızca adresin düşük baytına ekler. Elde oluşursa, yani $10F0 + $20 = $1110 gibi yüksek bayt değişirse, düzeltmek için bir döngü daha harcar. Örneğin LDA abs,X normalde 4 döngü sürer, sayfa geçerse 5 döngü. AbsoluteY ve IndirectIndexed’da da aynı kural geçerli. Saklama komutlarında (STA abs,X gibi) bu ekstra döngü sayfa geçmese bile her zaman harcanır. Dallanmalarda dal alınırsa +1, alınıp sayfa da geçilirse +1 daha eklenir. Bunu yapabilmek için get_operand_address’in sayfa geçildiğini dışarıya bildirmesi gerekiyor. cost de şu an decode_opcode içinde yerel bir değişken olduğu için CPU struct’ında bir döngü sayacına dönüşmeli.
//not: Şimdiden yapabileceğin, sonra işine yarayacak birkaç şey:

//Opcode tablosu: Her opcode için komut, adresleme modu, bayt sayısı ve döngü sayısını tek bir tabloda tut. Blok seviyesinde bu tablo kod çözücü ROM’una dönüşecek.
//Bayrak hesaplarını ayrı fonksiyonlara koy. Z/N güncellemeleri ve ADC’deki C/V hesabı gibi. Bunlar blok seviyesinde de aynen kullanılacak.
//nestest log’unu tutturmaya çalış. Her komuttan sonra PC, A, X, Y, P, SP ve döngü sayısını referans log’la aynı formatta yazdırırsan, geçişten sonra da aynı testi kullanabilirsin.

//The 6502 does not have any special support of hardware devices so they must be mapped to regions of memory in order to exchange data with the hardware latches.


struct CPU_6502{
    address_bus: u16, //The processor is little endian and expects addresses to be stored in memory least significant byte first.
    data_bus: u8,


    //registers
    accumulator: u8, //used all arithmetic and logical operations (with the exception of increments and decrements)
    //The contents of the accumulator can be stored and retrieved either from memory or the stack.


    status_register:u8,

    index_registers: [u8; 2],
    stack_pointer: u8, //holds the low 8 bits of the address of the next free location on the stack.
    //stacks location fixed in memory at $0100-$01FF, pushing bytes onto the stack decrements the stack pointer, and pulling bytes off the stack increments the stack pointer.
    //the stack pointer is initialized to $FF on reset, which means that the first byte pushed onto the stack will be stored at $01FF.
    //CPU doesnt detect stack overflow or underflow and it will most likely result in a crash.

    program_counter: u16,//The program counter is a 16 bit register which points to the next instruction to be executed. 
    //The value of program counter is modified automatically as instructions are executed.
    //The value of the program counter can be modified by executing a jump, a relative branch or..
    //..a subroutine call to another memory address or by returning from a subroutine or interrupt.





    memory:Memory
}
impl CPU_6502{
    //The 6502 has a 16-bit address bus, which allows it to address 64KB of memory. 
    

    

    //The 6502 has a 8-bit data bus, which allows it to read and write 8 bits of data at a time.
    fn data_bus_cycle(&mut self) {
        unimplemented!()
        //self.data_bus = self.read_memory(self.address_bus); 
    }







    //The 6502 has a 8-bit index registers, which are used for addressing memory.
    fn reg_x(&mut self) -> &mut u8 { &mut self.index_registers[0] }//hold counters or offsets for accessing memory. 
    //special function is to hold the offset for the zero page addressing mode.
    //The value of the X register can be loaded and saved in memory, compared with values held in memory or incremented and decremented.

    fn reg_y(&mut self) -> &mut u8 { &mut self.index_registers[1] }//same to x register without special function, but can be used for addressing memory.
    
    
    
    //The 6502 has a 8-bit stack pointer, which is used for subroutine calls and interrupts. it points next free location on stack
    
    
    
    
    //The 6502 has a 8-bit status register, which is used to indicate the state of the CPU.

    // Status register bits: N V - B D I Z C (bits 7 through 0).
    // Flags are stored in `status_register` using the bit assignments above.
    pub const N: u8 = 1 << 7; // Negative flag, is set if the result of the last operation had bit 7 set to a one.
    pub const V: u8 = 1 << 6; // Overflow flag, is set during arithmetic operations if the result has yielded an invalid 2's complement result (e.g. adding to positive numbers and ending up with a negative result: 64 + 64 => -128). It is determined by looking at the carry between bits 6 and 7 and between bit 7 and the carry flag.its set 
    pub const B: u8 = 1 << 4; // Break flag, is set when a BRK instruction executed and an interrupt has been generated to process it.
    pub const D: u8 = 1 << 3; // Decimal flag, while in decimal mode processor will obey the rules of BCD arithmetic for ADC and SBC instructions. this mode is not supported on the NES.
    pub const I: u8 = 1 << 2; // Interrupt  disable flag, CPU wont answer to interrupts if this flag is set.
    pub const Z: u8 = 1 << 1; // Zero flag, is set if the result of the last operation was zero
    pub const C: u8 = 1;      // Carry flag, positive if last operation produced a carry or borrow, negative if not. this condition is set during arithmetic operations and during logical shifts

    fn update_N(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::N; } else { self.status_register &= !CPU_6502::N; } }
    fn update_V(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::V; } else { self.status_register &= !CPU_6502::V; } }
    fn update_B(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::B; } else { self.status_register &= !CPU_6502::B; } }
    fn update_D(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::D; } else { self.status_register &= !CPU_6502::D; } }
    fn update_I(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::I; } else { self.status_register &= !CPU_6502::I; } }
    fn update_Z(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::Z; } else { self.status_register &= !CPU_6502::Z; } }
    fn update_C(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::C; } else { self.status_register &= !CPU_6502::C; } }

    fn read_N(&self) -> bool { (self.status_register & CPU_6502::N) != 0 }
    fn read_V(&self) -> bool { (self.status_register & CPU_6502::V) != 0 }
    fn read_B(&self) -> bool { (self.status_register & CPU_6502::B) != 0 }
    fn read_D(&self) -> bool { (self.status_register & CPU_6502::D) != 0 }
    fn read_I(&self) -> bool { (self.status_register & CPU_6502::I) != 0 }
    fn read_Z(&self) -> bool { (self.status_register & CPU_6502::Z) != 0 }
    fn read_C(&self) -> bool { (self.status_register & CPU_6502::C) != 0 }

    


    //The 6502 has a 16-bit program counter, which is used to keep track of the current instruction being executed.



    fn get_operand_address(&mut self, mode: &AddressingMode)-> OperandOfMode{
        match mode {
            &AddressingMode::Implicit =>{
                OperandOfMode::None
            }
            &AddressingMode::Accumulator =>{
                OperandOfMode::Accumulator
            }
            &AddressingMode::Immediate => {
                self.address_bus = self.program_counter; // it's supposed to already be in the program counter.
                self.program_counter=self.program_counter.wrapping_add(1);
                OperandOfMode::Memory
            }
            &AddressingMode::ZeroPage =>{ //In zero page mode only the least significant byte of the address is held in the instruction making it shorter by one byte (important for space saving) and one less memory fetch during execution (important for speed).
                let hi:u16= 0x0000;
                let lo:u16= self.read_next_byte() as u16;
                 //need to return 16 bit even if we have only 8 bit
                self.address_bus=hi|lo;
                OperandOfMode::Memory
                //An assembler will automatically select zero page addressing mode if the operand evaluates to a zero page address and the instruction supports the mode (not all do).
            }
            &AddressingMode::ZeroPageX =>{
                let hi:u16= 0x0000;

                self.address_bus =hi |(self.reg_x().wrapping_add( self.read_next_byte())) as u16 ;
                OperandOfMode::Memory
            }
            &AddressingMode::ZeroPageY =>{
                let hi: u16 = 0x0000;
                self.address_bus= hi | (self.reg_y().wrapping_add( self.read_next_byte())) as u16;
                OperandOfMode::Memory
            }

            &AddressingMode::Relative =>{
               self.address_bus=((self.read_next_byte() as i8) as u16).wrapping_add(self.program_counter);
               OperandOfMode::Memory
            }
            &AddressingMode::Absolute =>{
                let lo = self.read_next_byte() as u16;
                let hi = self.read_next_byte() as u16;
                self.address_bus=(hi << 8) | lo;
                OperandOfMode::Memory
            }
            &AddressingMode::AbsoluteX =>{
                let lo = self.read_next_byte() as u16;
                let hi = self.read_next_byte() as u16;
                self.address_bus=(hi << 8) | lo;
                self.address_bus= self.address_bus.wrapping_add(*self.reg_x() as u16);
                OperandOfMode::Memory
            }
            &AddressingMode::AbsoluteY =>{
                let lo = self.read_next_byte() as u16;
                let hi = self.read_next_byte() as u16;
                self.address_bus=(hi << 8) | lo;
                self.address_bus= self.address_bus.wrapping_add(*self.reg_y() as u16);
                OperandOfMode::Memory
            }
            &AddressingMode::Indirect=>{
                let lo: u8 = self.read_next_byte() ;
                let hi: u8 = self.read_next_byte();
                
                self.address_bus = self.memory.read(((hi as u16) << 8) | lo as u16) as u16 | ((self.memory.read(((hi as u16) << 8) | (lo.wrapping_add(1 as u8) as u16)) as u16) << 8);
               
                
                OperandOfMode::Memory
            }
            &AddressingMode::IndexedIndirect=>{
                let hi: u16 = 0x0000;
                let a= self.read_next_byte().wrapping_add(*self.reg_x());
                //memory.read yerine read_memory kullanmak daha profesyonel olabilir. belki öncesinde self.address busa kaydederiz.
                self.address_bus =   self.memory.read( hi | (a) as u16) as u16 | ((self.memory.read( hi |a.wrapping_add(1) as u16) as u16) << 8);

                OperandOfMode::Memory
            }  
            &AddressingMode::IndirectIndexed =>{
                 let hi: u16 = 0x0000;
                 let a: u8 = self.read_next_byte();
                 self.address_bus =   (self.memory.read( hi | a as u16) as u16 | ((self.memory.read( hi | a.wrapping_add(1) as u16) as u16) << 8)).wrapping_add(*self.reg_y() as u16);
                 

                 OperandOfMode::Memory

            }
            _ => panic!("Unknown mode: error of finding this mode"),

        }
        
    }


    pub fn decode_opcode(&mut self, opcode:u8){
        let mut cost :u8= 0;
        let mut bytes:u8= 0;
        
        match opcode{
            0xa9 => {
                cost = 2;
                bytes = 2;
                let operand: OperandOfMode = self.get_operand_address(&AddressingMode::Immediate);
                self.instruction_apply(operand, 23);
            }
            _ => {}
        }
    }


    pub fn instruction_apply(&mut self, operand: OperandOfMode, instruction: u8) {
        match instruction {
            0 => { //ADC
                
                let bit7_1: bool= (self.accumulator & (1 << 7)) >> 7 == 1;
                let mut is_overflow1: bool = false;
                let mut is_overflow2: bool = false;
                self.read_memory();


                (self.accumulator, is_overflow1) = self.accumulator.overflowing_add(self.data_bus);
                (self.accumulator, is_overflow2) = self.accumulator.overflowing_add(self.read_C() as u8);
                let bit7_2: bool = (self.accumulator & (1 << 7)) >> 7 == 1;
                
                self.update_Z(self.accumulator == 0);
                self.update_C(is_overflow1  | is_overflow2);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
                self.update_V((bit7_1 ==((self.data_bus & (1 << 7)) >> 7 == 1)) & (bit7_1 != bit7_2));
            },
            1 => { //AND
                self.accumulator = self.accumulator & self.read_memory();
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            2 => { //ASL
                
                let mut d:u8 = if matches!(operand, OperandOfMode::Accumulator){self.accumulator} else {self.read_memory()};

                let bit7: bool = (d & (1 << 7)) >> 7 == 1;
                d = d.wrapping_mul(2);
                self.update_C(bit7);
                self.update_Z(d == 0);
                self.update_N((d & (1 << 7)) >> 7 == 1);
                if matches!(operand, OperandOfMode::Accumulator){self.accumulator=d} else {self.write_memory(d);} 
            },
            3 => { //BCC
                if(self.read_C()==false) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}
            },
            4 => { //BCS
                if(self.read_C()==true) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}

            },
            5 => { //BEQ
                if(self.read_Z()==true) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}

            },
            6 => { //BIT
                let d = self.read_memory();
                self.update_Z((self.accumulator & d)==0);
                self.update_V((self.data_bus & (1<<6))>>6 ==1 );
                self.update_N((self.data_bus &(1<<7))>>7 ==1);
            },
            7 => {//BMI
                if(self.read_N()==true) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}


            },
            8 => {//BNE
                if(self.read_Z()==false) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}    
            },


            9 => { //BPL
                if(self.read_N()==false) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}
            },
            10 => {//BRK
                unimplemented!();
                self.update_B(true);
            },

            11 => { //BVC
                if(self.read_V()==false) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}
            },
            12 => { //BVS
                if(self.read_V()==true) {self.program_counter=self.address_bus} else {self.program_counter = self.program_counter.wrapping_add(1);}
            },
            13 => { //CLC
                self.update_C(false);
            },
            14 => { //CLD
                self.update_D(false);
            },
            15 => { //CLI
                self.update_I(false);
            },

            16 => { //CLV
                self.update_V(false);
            },
            17 => { //CMP
                let result: u8 = self.accumulator.wrapping_sub(self.read_memory());
                self.update_C(self.accumulator >= self.data_bus);
                self.update_Z(result == 0);
                self.update_N((result & (1 << 7)) >> 7 == 1);
            },
            18 => { //CPX
                let x = *self.reg_x();
                let result: u8 = (x).wrapping_sub(self.read_memory());
                self.update_C(x >= self.data_bus);
                self.update_Z(result == 0);
                self.update_N((result & (1 << 7)) >> 7 == 1);

            },
            19 => { //CPY
                let y = *self.reg_y();
                let result: u8 = (y).wrapping_sub(self.read_memory());
                self.update_C(y >= self.data_bus);
                self.update_Z(result == 0);
                self.update_N((result & (1 << 7)) >> 7 == 1);
            },
            20 => { //DEC
                self.data_bus=self.read_memory().wrapping_sub(1);
            },
            21 => { //DEX
                *self.reg_x() = (*self.reg_x()).wrapping_sub(1);
                let x = *self.reg_x();
                self.update_Z(x== 0);
                self.update_N((x & (1 << 7)) >> 7 == 1);
            },
            22 => { //DEY
                *self.reg_y() = (*self.reg_y()).wrapping_sub(1);
                let y = *self.reg_y();
                self.update_Z(y == 0);
                self.update_N((y & (1 << 7)) >> 7 == 1);
            },
            23 => { //EOR
                self.accumulator = self.accumulator ^ self.read_memory();
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            24 => { //INC
                self.data_bus=self.read_memory().wrapping_add(1);
                self.update_Z(self.data_bus == 0);
                self.update_N((self.data_bus & (1 << 7)) >> 7 == 1);
            },
            25 => { //INX
                *self.reg_x() = (*self.reg_x()).wrapping_add(1);
                let x = *self.reg_x();
                self.update_Z(x == 0);
                self.update_N((x & (1 << 7)) >> 7 == 1);
            },
            26 => { //INY
                *self.reg_y() = (*self.reg_y()).wrapping_add(1);
                let y=*self.reg_y();
                self.update_Z(y == 0);
                self.update_N((y & (1 << 7)) >> 7 == 1);
            },
            27 => { //JMP
                self.program_counter=self.address_bus;
            },
            28 => { //JSR

            },
            29 => { //LDA
                self.read_memory();    
                self.accumulator = self.data_bus;
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            30 => { //LDX
                self.read_memory();    
                *self.reg_x() = self.data_bus;
                let x=*self.reg_x();
                self.update_Z(x == 0);
                self.update_N((x & (1 << 7)) >> 7 == 1);
            },
            31 => { //LDY
                self.read_memory();    
                *self.reg_y() = self.data_bus;
                let y=*self.reg_y();
                self.update_Z(y == 0);
                self.update_N((y & (1 << 7)) >> 7 == 1);
            },
            32 => { //LSR
                let mut d:u8 = if matches!(operand, OperandOfMode::Accumulator){self.accumulator} else {self.read_memory()};

                self.update_C((d&1)==1); 
                d=d>>2;
                self.update_Z(d==0);
                self.update_N(false);
            },
            33 => { //NOP
                //NOP does nothing, but it takes time to execute. It is used to waste time or to align code in memory.a

            },
            34 => { //ORA
                self.accumulator = self.accumulator | self.read_memory();
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);

            },
            35 => { //PHA
                let stack:u16 = 0x0100;
                self.address_bus= stack.wrapping_add((self.stack_pointer as u16).wrapping_add(self.read_memory() as u16));
                self.write_memory(self.accumulator);
            },
            36 => { //PHP
                let status:u8= 1<<5;
                let stack:u16= 0x0100;
                self.address_bus=stack.wrapping_add((self.stack_pointer as u16).wrapping_add(self.read_memory() as u16));
                self.write_memory(status|(((self.read_N() as u8)<<7)|((self.read_V() as u8)<<6)|((self.read_B()as u8)<<4)|((self.read_D()as u8)<<3)|((self.read_I()as u8)<<2)|((self.read_Z()as u8)<<1)|((self.read_C()as u8))));

            },
            37 => { //PLA

                let stack:u16= 0x0100;
                let d = self.read_memory();
                self.accumulator= self.memory.read(stack.wrapping_add((self.stack_pointer as u16).wrapping_add(d as u16)));
            },
            38 => { //PLP
                
                let status:u8= 1<<5;
                let stack:u16= 0x0100;
                self.address_bus=stack.wrapping_add((self.stack_pointer as u16).wrapping_add(self.read_memory() as u16));
                let d:u8 = self.read_memory();
                self.update_N(((d>>7)&1)!=0);
                self.update_V(((d>>6)&1)!=0);
                self.update_B(((d>>4)&1)!=0);
                self.update_D(((d>>3)&1)!=0);
                self.update_I(((d>>2)&1)!=0);
                self.update_Z(((d>>1)&1)!=0);
                self.update_C(((d)&1)!=0);
            },
            39 => { //ROL
                
                let mut d:u8 = if matches!(operand, OperandOfMode::Accumulator){self.accumulator} else {self.read_memory()};

                let bit7: bool = (d & (1 << 7)) != 0;
                d = d.wrapping_mul(2);
                
                d=d|self.read_C() as u8;
                self.update_C(bit7);

                self.update_Z(d == 0);
                self.update_N((d & (1 << 7)) >> 7 == 1);

                if matches!(operand, OperandOfMode::Accumulator){self.accumulator=d} else {self.write_memory(d);}

            },
            40 => { //ROR
                let mut d:u8 = if matches!(operand, OperandOfMode::Accumulator){self.accumulator} else {self.read_memory()};

                let bit0: bool = (d & 1) != 0;
                d = d >> 1;
                d=d|((self.read_C() as u8)<<7);
                self.update_C(bit0);
                self.update_Z(d == 0);
                self.update_N((d & (1 << 7)) >> 7 == 1);
                if matches!(operand, OperandOfMode::Accumulator){self.accumulator=d} else {self.write_memory(d);}
            },
            41 => { //RTI

            },
            42 => { //RTS

            },
            43 => { //SBC
                self.accumulator=(self.accumulator.wrapping_sub(self.read_memory())).wrapping_sub(self.read_C() as u8);

                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            44 => { //SEC
                self.update_C(true);
            },
            45 => { //SED
                self.update_D(true);
            },
            46 => { //SEI
                self.update_I(true);
            },
            47 => { //STA
                self.write_memory( self.accumulator);
            },
            48 => { //STX
                self.data_bus = *self.reg_x();
                self.write_memory( self.data_bus);
            },
            49 => { //STY
                self.data_bus = *self.reg_y();
                self.write_memory( self.data_bus);
            },
            50 => { //TAX
                *self.reg_x() = self.accumulator;
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            51 => { //TAY
                *self.reg_y() = self.accumulator;
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            52 => { //TSX

            },
            53 => { //TXA
                self.accumulator = *self.reg_x();

                self.update_Z(self.accumulator==0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            54 => { //TXS
                *self.reg_x()=self.stack_pointer;
                let x = *self.reg_x();
                self.update_Z(x==0);
                self.update_N(((x&(1<<7))>>7)==1);
            },
            55 => { //TYA
                self.accumulator = *self.reg_y();

                self.update_Z(self.accumulator==0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },

    
            _ => panic!("Unknown instruction: {:02X}", instruction),
        }
    }










    // --- Mock helper functions (you will need to implement these based on your memory map) ---
    fn read_next_byte(&mut self) -> u8 {
        self.data_bus = self.memory.read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        self.data_bus
    }
    fn read_next_word(&mut self) -> u16{
        let lo = self.read_next_byte() as u16;
        let hi = self.read_next_byte() as u16;
        (hi << 8) | lo 
    }
    
    //fn read_next_word(&mut self) -> u16 {
    //    unimplemented!()
    //}
    
    fn read_memory(&mut self ) -> u8 {
        self.data_bus= self.memory.read(self.address_bus);
        return self.data_bus;
    }
    fn write_memory(&mut self,  data:u8){
        self.memory.write(self.address_bus, data);
    }




 

}



struct Memory{
    data: [u8; 65536],
    
    //first 0000-00FF(256 bytes) kısmına Zero Page,
    //The second 256 bytes ($0100–$01FF), reserved for the system stack.
    // The final 6 bytes ($FFFA–$FFFF):
    //non-maskable interrupt handler: $FFFA/B, power on reset: $FFFC/D, BRK/interrupt request: $FFFE/F
}
impl Memory{
    fn read(&self, addr: u16) -> u8{
        self.data[addr as usize]
    }
    fn write(&mut self, addr: u16, data:u8){
        self.data[addr as usize]= data;
    }
}




fn clock(){
    //The 6502 has a clock speed of 1 MHz, which means that it can execute 1 million instructions per second.
    //The 6502 has a clock cycle of 1 microsecond, which means that it takes 1 microsecond to execute one instruction.
}

pub enum OperandOfMode{
    None,
    Accumulator,
    Memory,

}
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,   
}

