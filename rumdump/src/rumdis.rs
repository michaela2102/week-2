type Umi = u32;

pub struct Field {
width: u32,
lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

enum Opcode {
    CMov,           // 0
    Load,           // 1
    Store,          // 2
    Add,            // 3
    Mul,            // 4
    Div,            // 5
    Nand,           // 6
    Halt,           // 7
    MapSegment,     // 8
    UnmapSegment,   // 9
    Output,         // 10
    Input,          // 11
    LoadProgram,    // 12
    LoadValue,      // 13
}

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

pub fn disassemble(inst: Umi) -> String {
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Load as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Store as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Add as u32 => {
            format!("r{} := r{} + r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        o if o == Opcode::Mul as u32 => {
            format!("r{} := r{} * r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Div as u32 => {
            format!("r{} := r{} / r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Nand as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Halt as u32 => {
            format!("halt")
        }
        o if o == Opcode::MapSegment as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::UnmapSegment as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Output as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::Input as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::LoadProgram as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        o if o == Opcode::LoadValue as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        }
        _=> format!("INVALID INSTRUCTION"),
    }
}