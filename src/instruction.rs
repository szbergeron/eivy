use smallvec::SmallVec;
use crate::ir;

pub struct InstructionID(usize);

pub struct InstructionSpecification<O: Opcode> {
    opcode: O,

    operands: SmallVec<[OperandSpecification; 4]>,

    lowering: Lowering,
}

pub struct Instruction<O: Opcode, A: Operand, const OPERAND_TYPICAL_COUNT: usize = 4> {
    opcode: O,
    operands: SmallVec<[A; OPERAND_TYPICAL_COUNT]>,
}

/*pub struct Opcode {
    bitlength: usize,

    // TODO: this should be a SmallVec or BigInt or
    // some other way of repr-ing large values.
    // It should also either define (or have implied)
    // the byte/bit ordering of the platform
    value: usize,
}*/

pub trait Opcode {
    fn throws(&self) -> bool;

    fn branches(&self) -> bool;

    fn is_call(&self) -> bool;
}

pub trait Operand {
}

pub struct OperandSpecification {
}

/// Defines the execution and side effects of this instruction
/// as imperative actions over the pre- and post- operands
pub struct Lowering {
    uops: SmallVec<[ir::Op; 5]>,
}
