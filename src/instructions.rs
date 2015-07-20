macro_rules! opcode {
    ($x:expr) => (($x >> 26) as u32);
}

macro_rules! register_source {
    ($x:expr) => ((($x >> 21) as u8 ) & 0x1f);
}
macro_rules! register_target {
    ($x:expr) => ((($x >> 16) as u8 ) & 0x1f);
}
macro_rules! register_destination {
    ($x:expr) => ((($x >> 11) as u8 ) & 0x1f);
}
macro_rules! shift_amount {
    ($x:expr) => ((($x >> 6)  as u8 ) & 0x1f);
}

macro_rules! function {
    ($x:expr) => ($x & 0x3f);
}
macro_rules! immediate {
    ($x:expr) => (($x & 0xffff) as i16);
}
macro_rules! target {
    ($x:expr) => (($x & 0x3ffffff) as i32);
}

type Register  = u8;
type Shamt     = u8;
type Target    = i32;
type Immediate = i16;
//                                                           Opcode    Function
#[derive(Debug, PartialEq)]
pub enum RType {                                    
    SLL,     //Logical Shift Left                            SPECIAL     0x00
    SRL,     //Logical Shift Right (0-extended)              SPECIAL     0x02
    SRA,     //Arithmetic Shift Right (sign-extended)        SPECIAL     0x03
    JR,      //Jump to Address in Register                   SPECIAL     0x08
    MFHI,    //Move from HI Register                         SPECIAL     0x10
    MFLO,    //Move from LO Register                         SPECIAL     0x12
    MULT,    //Multiply                                      SPECIAL     0x18
    MULTU,   //Unsigned Multiply                             SPECIAL     0x19
    DIV,     //Divide                                        SPECIAL     0x1A
    DIVU,    //Unsigned Divide                               SPECIAL     0x1B
    ADD,     //Add                                           SPECIAL     0x20
    ADDU,    //Add Unsigned                                  SPECIAL     0x21
    SUB,     //Subtract                                      SPECIAL     0x22
    SUBU,    //Unsigned Subtract                             SPECIAL     0x23
    AND,     //Bitwise AND                                   SPECIAL     0x24
    OR,      //Bitwise OR                                    SPECIAL     0x25
    XOR,     //Bitwise XOR (Exclusive-OR)                    SPECIAL     0x26
    NOR,     //Bitwise NOR (NOT-OR)                          SPECIAL     0x27
    SLT,     //Set to 1 if Less Than                         SPECIAL     0x2A
    SLTU,    //Set to 1 if Less Than Unsigned                SPECIAL     0x2B
    // MFC0, //Move from Coprocessor 0                       0x10         NA
}


#[derive(Debug, PartialEq)]
pub enum JType {
    J,       //Jump to Address                               0x02         NA
    JAL,     //Jump and Link                                 0x03         NA
}


#[derive(Debug, PartialEq)]
pub enum IType {
    SPECIAL, //                                              0x00         NA
    BEQ,     //Branch if Equal                               0x04         NA
    BNE,     //Branch if Not Equal                           0x05         NA
    ADDI,    //Add Immediate                                 0x08         NA
    ADDIU,   //Add Unsigned Immediate                        0x09         NA
    SLTI,    //Set to 1 if Less Than Immediate               0x0A         NA
    SLTIU,   //Set to 1 if Less Than Unsigned Immediate      0x0B         NA
    ANDI,    //Bitwise AND Immediate                         0x0C         NA
    ORI,     //Bitwise OR Immediate                          0x0D         NA
    LUI,     //Load Upper Immediate                          0x0F         NA
    LW,       //Load Word                                    0x23         NA
    LBU,     //Load Byte Unsigned                            0x24         NA
    LHU,     //Load Halfword Unsigned                        0x25         NA
    SB,      //Store Byte                                    0x28         NA
    SH,      //Store Halfword                                0x29         NA
    SW,      //Store Word                                    0x2B         NA

}
trait ToType {
    fn to_rtype(&self) -> RType;
    fn to_jtype(&self) -> JType;
    fn to_itype(&self) -> IType;
}

impl ToType for u32 {
    fn to_rtype(&self) -> RType {
        match function!(self) {
            0x00 => RType::SLL,
            0x02 => RType::SRL,
            0x03 => RType::SRA,
            0x08 => RType::JR,
            0x10 => RType::MFHI,
            0x12 => RType::MFLO,
            0x18 => RType::MULT,
            0x19 => RType::MULTU,
            0x1A => RType::DIV,
            0x1B => RType::DIVU,
            0x20 => RType::ADD,
            0x21 => RType::ADDU,
            0x22 => RType::SUB,
            0x23 => RType::SUBU,
            0x24 => RType::AND,
            0x25 => RType::OR,
            0x26 => RType::XOR,
            0x27 => RType::NOR,
            0x2A => RType::SLT,
            0x2B => RType::SLTU,
            _ => unimplemented!()
        }
    }

    fn to_jtype(&self) -> JType {
        match opcode!(self) {
            0x02 => JType::J,
            0x03 => JType::JAL,
            _ => unimplemented!()
        }
    }
    fn to_itype(&self) -> IType {
        match opcode!(self) {
            0x00 => IType::SPECIAL,
            0x04 => IType::BEQ,
            0x05 => IType::BNE,
            0x08 => IType::ADDI,
            0x09 => IType::ADDIU,
            0x0A => IType::SLTI,
            0x0B => IType::SLTIU,
            0x0C => IType::ANDI,
            0x0D => IType::ORI,
            0x0F => IType::LUI,
            0x23 => IType::LW,
            0x24 => IType::LBU,
            0x25 => IType::LHU,
            0x28 => IType::SB,
            0x29 => IType::SH,
            0x2B => IType::SW,
            _ => unimplemented!()
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Instruction {
    RType(IType, Register, Register, Register, Shamt, RType),   // opcode rs rt rd shift function
    JType(JType, Target),                                       // opcode address
    IType(IType, Register, Register, Immediate),                // opcode rs rt immediate
}

pub trait ToInstruction {
    fn to_instruction(&self) -> Instruction;
}

impl ToInstruction for u32 {
    fn to_instruction(&self) -> Instruction {
        match self.to_itype() {
            IType::SPECIAL => {
                let rs = register_source!(self);
                let rt = register_target!(self);
                let rd = register_destination!(self);
                let shamt = shift_amount!(self);
                let function: RType = function!(self).to_rtype();
                return Instruction::RType(IType::SPECIAL, rs, rt, rd, shamt, function);
            },
            IType => {
                let opcode = opcode!(self).to_itype();
                let rs = register_source!(self);
                let rt = register_target!(self);
                let immediate = immediate!(self);
                return Instruction::IType(opcode, rs, rt, immediate);
            }
        }
        match self.to_jtype() {
            JType::J | JType::JAL => {
                let opcode = opcode!(self).to_jtype();
                let target = target!(self);
                return Instruction::JType(opcode, target);
            }
        }

    }
}