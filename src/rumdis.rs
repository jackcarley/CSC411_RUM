type Umi = u32;
pub struct Field { width: u32,
lsb: u32, }
pub static RA: Field = Field {width: 3, lsb: 6}; 
pub static RB: Field = Field {width: 3, lsb: 3}; 
pub static RC: Field = Field {width: 3, lsb: 0}; 
pub static RL: Field = Field {width: 3, lsb: 25}; 
pub static VL: Field = Field {width: 25, lsb: 0}; 
pub static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 { 
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 { 
    println!("halt");
    (instruction >> OP.lsb) & mask(OP.width)
}

/* 
pub fn disassemble(inst: Umi) -> String { match get(&OP, inst) {
    o if o == Opcode::CMov as u32 => {
          format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        },
        */