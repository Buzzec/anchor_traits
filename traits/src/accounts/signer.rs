use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, DecodeAccounts, SingleAccount, ToAccountInfos, ToAccountMetas, ValidateAccounts,
};
use crate::traits::AccountsContext;
use derive_more::{Deref, DerefMut};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct Signer<T = AccountInfo>(pub T);
impl<T> ToAccountMetas for Signer<T>
where
    T: ToAccountMetas,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(&self.0, is_signer)
    }
}
impl<T> ToAccountInfos for Signer<T>
where
    T: ToAccountInfos,
{
    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(&self.0)
    }
}
impl<T> Accounts for Signer<T> where T: Accounts {}
impl<T> SingleAccount for Signer<T>
where
    T: SingleAccount,
{
    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self.0)
    }
}
impl<T, A> DecodeAccounts<A> for Signer<T>
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
impl<T, A> ValidateAccounts<A> for Signer<T>
where
    T: ValidateAccounts<A>,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult {
        if self.to_account_infos().all(|a| a.is_signer()) {
            T::validate(&mut self.0, accounts_context, arg)
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }
}
