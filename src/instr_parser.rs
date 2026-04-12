#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElaboratedOperand {
    Const(i32),
    Register(i8)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RTypeOpcode {
    Add,
    Sub,
    Slt,
    Sltu, 
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ITypeOpcode {
    Addi,
    Stli,
    Sltiu,
    Andi,
    Ori,
    Xori,
    Slli,
    Srli,
    Srai,
    Jalr,
    Lw
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UTypeOpcode {
    Lui,
    Auipc
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UJTypeOpcode {
    Jal
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum STypeOpcode {
    Sw
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SBTypeOpcode {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GTypeOpcode {
    Halt
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    RType {
        opcode: RTypeOpcode,
        rs1:    i32,
        rs2:    i32,
        rd:     i32
    },
    IType {
        opcode: ITypeOpcode,
        rs1:    i32,
        imm:    i32,
        rd:     i32
    },
    UType {
        opcode: UTypeOpcode,
        rd:     i32
    },
    UJType {
        opcode: UJTypeOpcode,
        rd:     i32
    },
    SType {
        opcode: STypeOpcode,
        rs1:    i32,
        rs2:    i32,
        imm:    i32
    },
    SBType {
        opcode: SBTypeOpcode,
        rs1:    i32,
        rs2:    i32,
        imm:    i32
    },
    GType {
        opcode: GTypeOpcode
    }
}



pub fn parse_instruction(instr: &str) -> Instruction {
    let no_commas  = instr.replace(",", " "); 
    let no_l_paren = no_commas.replace("(", " ");
    let no_r_paren = no_l_paren.replace(")", " ");
    let parsed_instr: Vec<&str> = no_r_paren.split_whitespace().collect();
    println!("{:?}", parsed_instr);

    match parsed_instr.as_slice() {
        ["add", operands @ ..] => {
            Instruction::RType { 
                opcode: RTypeOpcode::Add, 
                rd: operands[0].replace("x", "").parse().unwrap(), 
                rs1: operands[1].replace("x", "").parse().unwrap(), 
                rs2:  operands[2].replace("x", "").parse().unwrap(),
            }
        },
        ["addi", operands @ ..] => {
            Instruction::IType { 
                opcode: ITypeOpcode::Addi, 
                rs1:    operands[1].replace("x", "").parse().unwrap(), 
                imm:    operands[2].replace("x", "").parse().unwrap(), 
                rd:     operands[0].replace("x", "").parse().unwrap()
            }
        },
        ["halt", _operands @ .. ] => {
            Instruction::GType { opcode: GTypeOpcode::Halt }
        },
        _ => {
            println!("Skill issue");
            Instruction::RType { 
                            opcode: RTypeOpcode::Add, 
                            rd: 0,  
                            rs1: 7, 
                            rs2: 8,
                        }
        }
    }    
}

