use crate::ir::tests::ir_build_o1;
use crate::ir::cfg::CFG;
use crate::ir::{Operand, IRInst, Place, IRType};
use crate::ir::Operand::I32;
use std::collections::VecDeque;

#[test]
fn test_ir_builder() {
    let mut ir = ir_build_o1("fn main() {let a = 2 + 3 + 4 * 1;}").unwrap();

    let insts = VecDeque::from(vec![
        IRInst::load_data(Place::local("a_2".into(), IRType::I32), I32(9)),
        IRInst::Ret(Operand::Unit),
    ]);

    let func = ir.funcs.pop().unwrap();
    assert_eq!(insts, func.insts);

    let cfg = CFG::new(func);
    debug_assert_eq!("{\"a_2\": (0, I32)}", format!("{:?}", cfg.local_infos));

    assert_eq!(1, cfg.basic_blocks.len());
    let bb = cfg.basic_blocks.last().unwrap();
    assert_eq!(0, bb.id);
    assert!(bb.predecessors.is_empty());
    assert_eq!(2, bb.instructions.len());
    assert!(cfg.succ_of(0).is_empty());
}
