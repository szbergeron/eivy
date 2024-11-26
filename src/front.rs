use std::hash::Hash;

use crate::instruction::*;
use crate::memory::*;
use smallvec::SmallVec;

const OPERAND_TYPICAL_COUNT: usize = 4;

pub trait DecodeState {}

pub enum DecodeError {}

pub trait Decoder {
    type Ptr: Pointer;
    type DecodeState: Eq + Hash;
    type Opcode: Opcode;
    type Operand: Operand;
    const OPERAND_TYPICAL_COUNT: usize = 4;
    //type Operands = SmallVec<[Self::Operand; OPERAND_TYPICAL_COUNT]>;

    fn decode(
        &self,
        location: Self::Ptr,
        prior_state: Self::DecodeState,
    ) -> Result<
        (
            Self::DecodeState,
            Self::Ptr,
            Instruction<Self::Opcode, Self::Operand, { Self::OPERAND_TYPICAL_COUNT }>,
        ),
        DecodeError,
    >;
}
