use crate::error::AnchorResult;
use crate::traits::constraint::SupportsConstraint;
use crate::traits::AccountsContext;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, Debug)]
pub struct Executable;
impl SupportsConstraint<Executable> for AccountInfo {
    fn late_validation(
        &mut self,
        _constraint: &mut Executable,
        _context: &mut AccountsContext,
    ) -> AnchorResult {
        if self.executable() {
            Ok(())
        } else {
            Err(ProgramError::InvalidArgument)
        }
    }
}
