use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, DecodeAccounts, SingleAccount, ToAccountInfos, ToAccountMetas, ValidateAccounts,
};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::AccountsContext;
use derive_more::{Deref, DerefMut};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct Mut<T = AccountInfo>(pub T);

impl<T> ToAccountMetas for Mut<T>
where
    T: ToAccountMetas,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(&self.0, is_signer)
    }
}
impl<T> ToAccountInfos for Mut<T>
where
    T: ToAccountInfos,
{
    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(&self.0)
    }
}
impl<T> Accounts for Mut<T> where T: Accounts {}
impl<T> SingleAccount for Mut<T>
where
    T: SingleAccount,
{
    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self.0)
    }
}
impl<T, A> DecodeAccounts<A> for Mut<T>
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
impl<T, A> ValidateAccounts<A> for Mut<T>
where
    T: ValidateAccounts<A>,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        if self.to_account_infos().all(|a| a.is_writable()) {
            T::validate(&mut self.0, accounts_context, arg)
        } else {
            Err(ProgramError::AccountBorrowFailed)
        }
    }
}
impl<T, C> SupportsConstraint<C> for Mut<T>
where
    T: SupportsConstraint<C>,
    C: Constraint,
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
