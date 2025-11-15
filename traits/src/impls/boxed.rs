use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, DecodeAccounts, SingleAccount, ToAccountInfos, ToAccountMetas, ValidateAccounts,
};
use crate::traits::AccountsContext;
use alloc::boxed::Box;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;

impl<T> ToAccountInfos for Box<T>
where
    T: ToAccountInfos,
{
    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(self)
    }
}
impl<T> ToAccountMetas for Box<T>
where
    T: ToAccountMetas,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(self, is_signer)
    }
}
impl<T> Accounts for Box<T> where T: Accounts {}
impl<T> SingleAccount for Box<T>
where
    T: SingleAccount,
{
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
