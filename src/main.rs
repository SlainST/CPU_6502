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
//The 6502 does not have any special support of hardware devices so they must be mapped to regions of memory in order to exchange data with the hardware latches.


struct CPU_6502{
    address_bus: u16, //The processor is little endian and expects addresses to be stored in memory least significant byte first.
    data_bus: u8,


    //registers
    accumulator: u8, //used all arithmetic and logical operations (with the exception of increments and decrements)
    //The contents of the accumulator can be stored and retrieved either from memory or the stack.


    index_registers: [u8; 2],
    stack_pointer: u8, //holds the low 8 bits of the address of the next free location on the stack.
    //stacks location fixed in memory at $0100-$01FF, pushing bytes onto the stack decrements the stack pointer, and pulling bytes off the stack increments the stack pointer.
    //the stack pointer is initialized to $FF on reset, which means that the first byte pushed onto the stack will be stored at $01FF.
    //CPU doesnt detect stack overflow or underflow and it will most likely result in a crash.


    status_register: u8, 




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



    //The 6502 has a 8-bit accumulator, which is used for arithmetic and logic operations.
    fn accumulator(&self) -> u8 { self.accumulator }



    //The 6502 has a 8-bit index registers, which are used for addressing memory.
    fn reg_x(&self) -> u8 { self.index_registers[0] }//hold counters or offsets for accessing memory. 
    //special function is to hold the offset for the zero page addressing mode.
    //The value of the X register can be loaded and saved in memory, compared with values held in memory or incremented and decremented.

    fn reg_y(&self) -> u8 { self.index_registers[1] }//same to x register without special function, but can be used for addressing memory.
    
    
    
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
    fn BRK(&mut self) { self.status_register |= CPU_6502::B; } //Set Break Command instruction
    fn SED(&mut self) { self.status_register |= CPU_6502::D; } //Set Decimal Mode instruction
    fn CLD(&mut self) { self.status_register &= !CPU_6502::D; } //Clear Decimal Mode instruction
    fn SEI(&mut self) { self.status_register |= CPU_6502::I; } //Set Interrupt Disable instruction
    fn CLI(&mut self) { self.status_register &= !CPU_6502::I; }//Clear Interrupt Disable instruction
    fn update_Z(&mut self, value: bool){ if value == true { self.status_register |= CPU_6502::Z; } else { self.status_register &= !CPU_6502::Z; } }
    fn SEC(&mut self) { self.status_register |= CPU_6502::C; } //set carry flag instruction
    fn CLC(&mut self) { self.status_register &= !CPU_6502::C; } //clear carry flag instruction
    fn read_N(&self) -> bool { (self.status_register & CPU_6502::N) != 0 }
    fn read_V(&self) -> bool { (self.status_register & CPU_6502::V) != 0 }
    fn read_B(&self) -> bool { (self.status_register & CPU_6502::B) != 0 }
    fn read_D(&self) -> bool { (self.status_register & CPU_6502::D) != 0 }
    fn read_I(&self) -> bool { (self.status_register & CPU_6502::I) != 0 }
    fn read_Z(&self) -> bool { (self.status_register & CPU_6502::Z) != 0 }
    fn read_C(&self) -> bool { (self.status_register & CPU_6502::C) != 0 }

    


    //The 6502 has a 16-bit program counter, which is used to keep track of the current instruction being executed.
    



    ///instructions
    fn ACD(&mut self){ // 0
    }
    fn AND(&mut self){ // 1
    }
    fn ASL(&mut self){ // 2
    }
    fn BCC(&mut self){ // 3
    }
    fn BCS(&mut self){ // 4
    }
    fn BEQ(&mut self){ // 6
    }
    fn BIT(&mut self){ // 6
    }
    fn BPL(&mut self){ // 7
    }
    fn BVC(&mut self){ // 8
    }
    fn BVS(&mut self){ // 9
    }
    fn CLV(&mut self){ // 10
    }
    fn CMP(&mut self){ // 11
    }
    fn CPX(&mut self){ // 12
    }
    fn CPY(&mut self){ // 13
    }
    fn DEC(&mut self){ // 14
    }
    fn DEX(&mut self){ // 15
    }
    fn DEY(&mut self){ // 16
    }
    fn EOR(&mut self){ // 17
    }
    fn INC(&mut self){ // 18
    }
    fn INX(&mut self){ // 19
    }
    fn INY(&mut self){ // 20
    }
    fn JMP(&mut self){ // 21
    }
    fn JSR(&mut self){ // 22
    }
    fn LDA(&mut self, addr: u16){ // 23
        self.accumulator = self.read_memory(addr);
        self.update_Z(self.accumulator == 0);
        self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
    }
    fn LDX(&mut self){ // 24
    }
    fn LDY(&mut self){ // 25
    }
    fn LSR(&mut self){ // 26
    }
    fn NOP(&mut self){ // 27
    }
    fn ORA(&mut self){ // 28
    }
    fn PHA(&mut self){ // 29
    }
    fn PHP(&mut self){ // 30
    }
    fn PLA(&mut self){ // 31
    }
    fn PLP(&mut self){ // 32
    }
    fn ROL(&mut self){ // 33
    }
    fn ROR(&mut self){ // 34
    }
    fn RTI(&mut self){ // 35
    }
    fn RTS(&mut self){ // 36
    }
    fn SBC(&mut self){ // 37
    }
    fn STA(&mut self){ // 38
    }
    fn STX(&mut self){ // 39
    }
    fn STY(&mut self){ // 40
    }
    fn TAX(&mut self){ // 41
    }
    fn TAY(&mut self){ // 42
    }
    fn TSX(&mut self){ // 43
    }
    fn TXA(&mut self){ // 44
    }
    fn TXS(&mut self){ // 45
    }
    fn TYA(&mut self){ // 46
    }


    fn get_operand_address(&mut self, mode: &AddressingMode)-> u16{
        match mode {
            &AddressingMode::Implicit =>{
                unimplemented!()
            }
            &AddressingMode::Accumulator =>{
                unimplemented!()
            }
            &AddressingMode::Immediate => {
                self.address_bus = self.program_counter; // it's supposed to already be in the program counter.
                self.program_counter += 1;
            }
            &AddressingMode::ZeroPage =>{
                unimplemented!() 
                 //need to return 16 bit even if we have only 8 bit
            }
            &AddressingMode::ZeroPageX =>{
                unimplemented!()
            }
            &AddressingMode::ZeroPageY =>{
                unimplemented!()
            }
            &AddressingMode::Relative =>{
                unimplemented!()
            }
            &AddressingMode::Absolute =>{
                let lo = self.read_next_byte() as u16;
                let hi = self.read_next_byte() as u16;
                self.address_bus=(hi << 8) | lo;
            }
            &AddressingMode::AbsoluteX =>{
                unimplemented!()
            }
            &AddressingMode::AbsoluteY =>{
                unimplemented!()
            }
            &AddressingMode::Indirect=>{
                unimplemented!()
            }
            &AddressingMode::IndexedIndirect=>{
                unimplemented!()
            }  
            &AddressingMode::IndirectIndexed =>{
                unimplemented!()
            }
            _ => panic!("Unknown mode: error of finding this mode"),

        }
        self.address_bus
    }


    pub fn decode_opcode(&mut self, opcode:u8){
        let mut cost :u8= 0;
        let mut bytes:u8= 0;
        
        match opcode{
            0xa9 => {
                cost = 2;
                bytes = 2;
                let mut addr:u16= self.get_operand_address(&AddressingMode::Immediate);
                self.instruction_apply(addr, 23);
            }
            _ => {}
        }

        
    }
    pub fn instruction_apply(&mut self, operand_address: u16, instruction: u8) {
        match instruction {
            23 => {
                self.read_memory(operand_address);
                self.accumulator= self.data_bus;
                self.update_Z(self.accumulator == 0);
                self.update_N((self.accumulator & (1 << 7)) >> 7 == 1);
            },
            _ => panic!("Unknown instruction: {:02X}", instruction),
        }
    }











    // --- Mock helper functions (you will need to implement these based on your memory map) ---
    fn read_next_byte(&mut self) -> u8 {
        let data = self.memory.read(self.program_counter);
        self.program_counter +=1;
        data
    }
    
    fn read_next_word(&mut self) -> u16 {
        unimplemented!()
    }
    
    fn read_memory(&mut self, addr: u16) {
        self.data_bus= self.memory.read(addr);
    }
    fn write_memory(&mut self, addr:u16, data:u8){
        self.memory.write(addr, data);
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
