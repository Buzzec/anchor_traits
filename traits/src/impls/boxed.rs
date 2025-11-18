use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::AccountsContext;
use alloc::boxed::Box;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;

impl<T> Accounts for Box<T>
where
    T: Accounts,
{
    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(self)
    }

    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(self, is_signer)
    }
}
unsafe impl<T> SingleAccount for Box<T>
where
    T: SingleAccount,
{
    type Mutable = T::Mutable;
    type CanSign = T::CanSign;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(self)
    }
}
impl<T, A> DecodeAccounts<A> for Box<T>
where
    T: DecodeAccounts<A>,
{
    #[inline]
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: A,
    ) -> AnchorResult<Self> {
        T::try_accounts(accounts_context, accounts, arg).map(Box::new)
    }

    #[inline]
    fn size_hint() -> (usize, Option<usize>) {
        T::size_hint()
    }
}
impl<T, A> ValidateAccounts<A> for Box<T>
where
    T: ValidateAccounts<A>,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        T::validate(self, accounts_context, arg)
    }
}
impl<T, A> CleanupAccounts<A> for Box<T>
where
    T: CleanupAccounts<A>,
{
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        T::cleanup(self, accounts_context, arg)
    }
}
