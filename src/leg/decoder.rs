use crate::prelude::*;

pub struct Legv8Decoder {
}

pub struct Legv8Pointer {
    addr: u32,
}

pub enum Legv8Opcode {
}

impl Opcode for Legv8Opcode {
    fn throws(&self) -> bool {
        todo!()
    }

    fn branches(&self) -> bool {
        todo!()
    }

    fn is_call(&self) -> bool {
        todo!()
    }
}

enum Legv8Operand {
}

impl Operand for Legv8Operand {
}

impl Pointer for Legv8Pointer {
    fn address(&self) -> usize {
        self.addr as usize
    }
}

impl Decoder for Legv8Decoder {
    type Ptr = Legv8Pointer;
    type DecodeState = ();
    type Opcode = Legv8Opcode;
    type Operand = Legv8Operand;

    const OPERAND_TYPICAL_COUNT: usize = 4;

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
        > {
        todo!()
    }
}
