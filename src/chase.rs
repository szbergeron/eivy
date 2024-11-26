use smallvec::SmallVec;

use crate::{
    front::Decoder,
    instruction::{Instruction, Opcode},
};

/// Chaser effectively "drives" initial disassembly.
///
/// It tries exploring out and building a coverage map from every `entry()` call
/// Note that, because of the hazards present in degenerate assembly, this
/// coverage map is nearly guaranteed to be partial.
///
/// Straight runs of instructions may (in the absense of interrupts) be
/// relatively straightforwardly explored, but note that hidden behind any
/// opaque `call` or branching instruction could be horrible stack and
/// register munging
struct Chaser<D: Decoder> {
    decoder: D,
}

impl<D: Decoder> Chaser<D>
where
    [(); D::OPERAND_TYPICAL_COUNT]:,
{
    /// This is the primary/sole entry for Chaser,
    /// and should be called when first loading a program/exec'ing into it,
    /// as well as at <TODO: figure out the other feasible points?
    /// this is pretty much just resume points after hazards, but need
    /// to encode that>
    pub fn enter(&mut self, at: D::Ptr, with: D::DecodeState) -> Result<(), ()> {
        let (s, n, i) = self.decoder.decode(at, with).map_err(|_| ())?;

        todo!()
    }

    /// This tries to detect if this instruction presents a chase
    /// hazard that can not be rectified or bypassed.
    ///
    /// This mostly has to do with if the instruction could hide a
    /// modification of the instruction ptr, or could trigger an interrupt
    /// that is required to (or may always) diverge.
    fn is_fatal_hazard(
        &self,
        o: &D::Opcode,
        ps: &Instruction<D::Opcode, D::Operand, { D::OPERAND_TYPICAL_COUNT }>,
    ) -> bool {
        todo!()
    }

    /// This function takes, as input, a known entry or branch target
    /// Behavior is undefined and unpredictable if a pointer not
    /// satisfying the above is passed.
    ///
    /// If this function determines that this function _must_ be a leaf,
    /// it will return True-With-Certainty
    fn is_leaf(
        &mut self,
        p: D::Ptr
    ) -> ! {
        todo!()
    }
}
