/// Enum containing all possible instruction types.
#[derive(Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Opcode {
    #[default]
    Return,
}

pub struct InstrId(pub u32);

// BB instruction
#[derive(Default)]
pub struct Instr {
    pub inputs: Vec<InstrId>,
    pub outputs: Vec<InstrId>,
    pub opcode: Opcode,
}

pub struct InstrArena {
    pub instrs: Vec<Instr>,
}

impl Instr {
    pub fn new(opcode: Opcode,  inputs:  Vec<InstrId>, outputs:  Vec<InstrId>) -> Self {
        Instr { opcode, inputs, outputs }
    }

    pub fn set_def(&mut self, item: InstrId, index: i32) {
        self.inputs[index as usize] = item;
    }

    pub fn set_use(&mut self, item: InstrId, index: i32) {
        self.outputs[index as usize] = item;
    }

}

impl InstrArena {
    pub fn get(&self, id: InstrId) -> &Instr {
        &self.instrs[id.0 as usize]
    }

    pub fn push(&mut self, instr: Instr) -> InstrId {
        let id = InstrId(self.instrs.len() as u32);
        self.instrs.push(instr);
        id
    }
}
#[cfg(test)]
mod tests {
    use crate::cfg::instr::{Instr, Opcode};

    fn new_instruction() -> Instr {
        Instr::new(Opcode::Return, vec![], vec![])
    }
    #[test]
    fn test_kind() {
        let ins = new_instruction();

        assert_eq!(ins.opcode, Opcode::Return);
    }

    #[test]
    fn test_size() {
        let ins = new_instruction();

        assert_eq!(size_of::<Instr>(), 56);
    }
}
