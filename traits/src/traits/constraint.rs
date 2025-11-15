use crate::error::AnchorResult;
use crate::traits::account::Accounts;
use crate::traits::builder::BuilderStart;
use crate::traits::AccountsContext;

#[allow(unused_variables)]
pub trait SupportsConstraint<C: Constraint>: Accounts {
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

pub trait Constraint {
    type Builder: BuilderStart<Output = Self>;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }
}
