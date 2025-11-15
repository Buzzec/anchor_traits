use crate::error::AnchorResult;
use crate::traits::builder::{Builder, BuilderField, BuilderFinish, BuilderStart};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::AccountsContext;
use crate::util::EmptyField;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;

pub struct Address(pub Pubkey);
impl Constraint for Address {
    type Builder = AddressBuilder<EmptyField>;
}

pub struct AddressBuilder<A>(pub A);
pub struct AddressField0;
impl<A> Builder for AddressBuilder<A> {
    type Output = Address;
}
impl BuilderStart for AddressBuilder<EmptyField> {
    #[inline]
    fn new() -> Self {
        AddressBuilder(EmptyField)
    }
}
impl BuilderField<AddressField0> for AddressBuilder<EmptyField> {
    type FieldType = Pubkey;
    type AfterSet = AddressBuilder<Pubkey>;

    fn set_field(self, field: Self::FieldType) -> Self::AfterSet {
        AddressBuilder(field)
    }
}
impl BuilderFinish for AddressBuilder<Pubkey> {
    fn finish(self) -> Self::Output {
        Address(self.0)
    }
}

impl SupportsConstraint<Address> for AccountInfo {
    fn late_validation(
        &mut self,
        constraint: &mut Address,
        _context: &mut AccountsContext,
    ) -> AnchorResult {
        if self.key() == &constraint.0 {
            Ok(())
        } else {
            Err(ProgramError::InvalidArgument)
        }
    }
}
