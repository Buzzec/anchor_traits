use crate::error::AnchorResult;
use crate::traits::builder::{Builder, BuilderFinish, BuilderStart};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::AccountsContext;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, Debug)]
pub struct Signer;

impl Constraint for Signer {
    type Builder = Self;
}
impl Builder for Signer {
    type Output = Self;
}
impl BuilderStart for Signer {
    #[inline]
    fn new() -> Self {
        Signer
    }
}
impl BuilderFinish for Signer {
    #[inline]
    fn finish(self) -> Self::Output {
        self
    }
}
impl SupportsConstraint<Signer> for AccountInfo {
    fn late_validation(
        &mut self,
        _constraint: &mut Signer,
        _context: &mut AccountsContext,
    ) -> AnchorResult {
        if self.is_signer() {
            Ok(())
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }
}
