use crate::error::AnchorResult;
use crate::traits::account::Accounts;
use crate::traits::AccountsContext;

#[allow(unused_variables)]
pub trait SupportsConstraint<C>: Accounts {
    fn early_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        Ok(())
    }

    fn late_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        Ok(())
    }

    fn cleanup(&mut self, constraint: &mut C, context: &mut AccountsContext) -> AnchorResult {
        Ok(())
    }
}
