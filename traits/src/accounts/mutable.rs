use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::constraint::SupportsConstraint;
use crate::traits::maybe_bool::True;
use crate::traits::AccountsContext;
use derive_more::{Deref, DerefMut};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

pub type Mut<T = AccountInfo> = Mutability<T, true>;
pub type ReadOnly<T = AccountInfo> = Mutability<T, false>;

#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct Mutability<T, const IS_MUT: bool>(pub T);

impl<T, const IS_MUT: bool> Accounts for Mutability<T, IS_MUT>
where
    T: Accounts,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(&self.0, is_signer)
    }

    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(&self.0)
    }
}
unsafe impl<T> SingleAccount for Mutability<T, true>
where
    T: SingleAccount,
{
    type Mutable = True;
    type CanSign = T::CanSign;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self.0)
    }
}
unsafe impl<T> SingleAccount for Mutability<T, false>
where
    T: SingleAccount,
{
    type Mutable = True;
    type CanSign = T::CanSign;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self.0)
    }
}
impl<T, A, const IS_MUT: bool> DecodeAccounts<A> for Mutability<T, IS_MUT>
where
    T: DecodeAccounts<A>,
{
    #[inline]
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: A,
    ) -> AnchorResult<Self> {
        T::try_accounts(accounts_context, accounts, arg).map(Self)
    }

    #[inline]
    fn size_hint() -> (usize, Option<usize>) {
        T::size_hint()
    }
}
impl<T, A, const IS_MUT: bool> ValidateAccounts<A> for Mutability<T, IS_MUT>
where
    T: ValidateAccounts<A>,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        if self.to_account_infos().all(|a| {
            if const { IS_MUT } {
                a.is_writable()
            } else {
                !a.is_writable()
            }
        }) {
            T::validate(&mut self.0, accounts_context, arg)
        } else {
            Err(ProgramError::AccountBorrowFailed)
        }
    }
}
impl<T, A, const IS_MUT: bool> CleanupAccounts<A> for Mutability<T, IS_MUT>
where
    T: CleanupAccounts<A>,
{
    #[inline]
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        T::cleanup(&mut self.0, accounts_context, arg)
    }
}
impl<T, C, const IS_MUT: bool> SupportsConstraint<C> for Mutability<T, IS_MUT>
where
    T: SupportsConstraint<C>,
{
    fn early_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        T::early_validation(&mut self.0, constraint, context)
    }

    fn late_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        T::late_validation(&mut self.0, constraint, context)
    }

    fn cleanup(&mut self, constraint: &mut C, context: &mut AccountsContext) -> AnchorResult {
        T::cleanup(&mut self.0, constraint, context)
    }
}
