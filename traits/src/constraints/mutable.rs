use crate::error::AnchorResult;
use crate::traits::builder::{Builder, BuilderFinish, BuilderStart};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::AccountsContext;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, Debug)]
pub struct Mut;

impl Constraint for Mut {
    type Builder = Self;
}
impl Builder for Mut {
    type Output = Self;
}
impl BuilderStart for Mut {
    #[inline]
    fn new() -> Self {
        Mut
    }
}
impl BuilderFinish for Mut {
    #[inline]
    fn finish(self) -> Self::Output {
        self
    }
}
impl SupportsConstraint<Mut> for AccountInfo {
    fn late_validation(
        &mut self,
        _constraint: &mut Mut,
        _context: &mut AccountsContext,
    ) -> AnchorResult {
        if self.is_writable() {
            Ok(())
        } else {
            Err(ProgramError::AccountBorrowFailed)
        }
    }
}
