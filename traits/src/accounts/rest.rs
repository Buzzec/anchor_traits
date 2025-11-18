use crate::error::AnchorResult;
use crate::traits::account::{Accounts, CleanupAccounts, DecodeAccounts, ValidateAccounts};
use crate::traits::constraint::{Constraint, SupportsConstraint};
use crate::traits::AccountsContext;
use alloc::vec::Vec;
use derive_more::{Deref, DerefMut};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Rest<T>(pub Vec<T>);
impl<T> Accounts for Rest<T>
where
    T: Accounts,
{
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        self.iter().flat_map(T::to_account_infos)
    }

    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        self.iter()
            .flat_map(move |a| T::to_account_metas(a, is_signer))
    }
}
impl<T, A> DecodeAccounts<A> for Rest<T>
where
    T: DecodeAccounts<A>,
    A: Clone,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: A,
    ) -> AnchorResult<Self> {
        let mut accounts = accounts.peekable();

        let mut out = Vec::with_capacity(T::size_hint().0);
        while accounts.peek().is_some() {
            out.push(T::try_accounts(
                accounts_context,
                &mut accounts,
                arg.clone(),
            )?);
        }

        Ok(Self(out))
    }
}
impl<T, A> ValidateAccounts<A> for Rest<T>
where
    T: ValidateAccounts<A>,
    A: Clone,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        for t in self.iter_mut() {
            T::validate(t, accounts_context, arg.clone())?;
        }
        Ok(())
    }
}
impl<T, A> CleanupAccounts<A> for Rest<T>
where
    T: CleanupAccounts<A>,
    A: Clone,
{
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        for t in self.iter_mut() {
            T::cleanup(t, accounts_context, arg.clone())?;
        }
        Ok(())
    }
}
impl<T, C> SupportsConstraint<C> for Rest<T>
where
    T: SupportsConstraint<C>,
    C: Constraint,
{
    fn early_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        for t in self.iter_mut() {
            T::early_validation(t, constraint, context)?;
        }
        Ok(())
    }

    fn late_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        for t in self.iter_mut() {
            T::late_validation(t, constraint, context)?;
        }
        Ok(())
    }

    fn cleanup(&mut self, constraint: &mut C, context: &mut AccountsContext) -> AnchorResult {
        for t in self.iter_mut() {
            T::cleanup(t, constraint, context)?;
        }
        Ok(())
    }
}
