// Basic block
use crate::cfg::instr::Instr;

pub struct BB {
    preds: Vec<BB>,
    succs: Vec<BB>,
    // linear flat list of instructions
    instrs: Vec<Instr>,
}
