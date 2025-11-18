use crate::error::AnchorResult;
use crate::traits::constraint::SupportsConstraint;
use crate::traits::AccountsContext;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;

#[derive(Copy, Clone, Debug)]
pub struct Owner(pub Pubkey);

impl SupportsConstraint<Owner> for AccountInfo {
    fn late_validation(
        &mut self,
        constraint: &mut Owner,
        _context: &mut AccountsContext,
    ) -> AnchorResult {
        if self.key() == &constraint.0 {
            Ok(())
        } else {
            Err(ProgramError::InvalidArgument)
        }
    }
}
