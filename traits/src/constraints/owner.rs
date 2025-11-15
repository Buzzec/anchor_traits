use crate::error::AnchorResult;
use crate::traits::builder::{Builder, BuilderField, BuilderFinish, BuilderStart};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::AccountsContext;
use crate::util::EmptyField;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;

#[derive(Copy, Clone, Debug)]
pub struct Owner(pub Pubkey);
impl Constraint for Owner {
    type Builder = OwnerBuilder<EmptyField>;
}

pub struct OwnerBuilder<A>(pub A);
pub struct OwnerField0;
impl<A> Builder for OwnerBuilder<A> {
    type Output = Owner;
}
impl BuilderStart for OwnerBuilder<EmptyField> {
    #[inline]
    fn new() -> Self {
        OwnerBuilder(EmptyField)
    }
}
impl BuilderField<OwnerField0> for OwnerBuilder<EmptyField> {
    type FieldType = Pubkey;
    type AfterSet = OwnerBuilder<Pubkey>;

    fn set_field(self, field: Self::FieldType) -> Self::AfterSet {
        OwnerBuilder(field)
    }
}
impl BuilderFinish for OwnerBuilder<Pubkey> {
    fn finish(self) -> Self::Output {
        Owner(self.0)
    }
}

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
