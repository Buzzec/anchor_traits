use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::program::ProgramId;
use crate::traits::AccountsContext;
use core::fmt::Debug;
use core::marker::PhantomData;
use derive_more::{Deref, DerefMut};
use derive_where::derive_where;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[derive_where(Clone; T: Clone)]
#[derive_where(Copy; T: Copy)]
#[derive_where(Debug; T: Debug)]
#[derive(Deref, DerefMut)]
pub struct Program<P: ProgramId, T = AccountInfo> {
    #[deref]
    #[deref_mut]
    info: T,
    _program: PhantomData<fn() -> P>,
}
impl<P: ProgramId, T> Program<P, T> {
    #[inline]
    pub fn new_unchecked(info: T) -> Self {
        Self {
            info,
            _program: PhantomData,
        }
    }
}
impl<P: ProgramId, T> Accounts for Program<P, T>
where
    T: Accounts,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(&self.info, is_signer)
    }

    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(&self.info)
    }
}
unsafe impl<P: ProgramId, T> SingleAccount for Program<P, T>
where
    T: SingleAccount,
{
    type Mutable = T::Mutable;
    type CanSign = T::CanSign;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self.info)
    }
}
impl<P: ProgramId, T, A> DecodeAccounts<A> for Program<P, T>
where
    T: DecodeAccounts<A>,
{
    #[inline]
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: A,
    ) -> AnchorResult<Self> {
        T::try_accounts(accounts_context, accounts, arg).map(Self::new_unchecked)
    }

    #[inline]
    fn size_hint() -> (usize, Option<usize>) {
        T::size_hint()
    }
}
impl<P: ProgramId, T, A> ValidateAccounts<A> for Program<P, T>
where
    T: ValidateAccounts<A> + SingleAccount,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        if self.info.key() == &P::ID {
            T::validate(&mut self.info, accounts_context, arg)
        } else {
            Err(ProgramError::IncorrectProgramId)
        }
    }
}
impl<P: ProgramId, T, A> CleanupAccounts<A> for Program<P, T>
where
    T: CleanupAccounts<A>,
{
    #[inline]
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        T::cleanup(&mut self.info, accounts_context, arg)
    }
}
impl<P: ProgramId, T, C> SupportsConstraint<C> for Program<P, T>
where
    T: SupportsConstraint<C>,
    C: Constraint,
{
    fn early_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        T::early_validation(&mut self.info, constraint, context)
    }

    fn late_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        T::late_validation(&mut self.info, constraint, context)
    }

    fn cleanup(&mut self, constraint: &mut C, context: &mut AccountsContext) -> AnchorResult {
        T::cleanup(&mut self.info, constraint, context)
    }
}
