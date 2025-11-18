use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::AccountsContext;
use crate::util::try_map_array_init;
use array_init::try_array_init;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;

impl<T, const N: usize> Accounts for [T; N]
where
    T: Accounts,
{
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        self.iter().flat_map(T::to_account_infos)
    }

    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        self.iter()
            .flat_map(move |t| T::to_account_metas(t, is_signer))
    }
}
unsafe impl<T> SingleAccount for [T; 1]
where
    T: SingleAccount,
{
    type Mutable = T::Mutable;
    type CanSign = T::CanSign;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self[0])
    }
}
impl<T, const N: usize, A> DecodeAccounts<[A; N]> for [T; N]
where
    T: DecodeAccounts<A>,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: [A; N],
    ) -> AnchorResult<Self> {
        try_map_array_init(arg, |arg| T::try_accounts(accounts_context, accounts, arg))
    }

    fn size_hint() -> (usize, Option<usize>) {
        if const { N == 0 } {
            (0, Some(0))
        } else {
            let t_size_hint = T::size_hint();
            (t_size_hint.0 * N, t_size_hint.1.map(|v| v * N))
        }
    }
}
impl<T, const N: usize, A> DecodeAccounts<(A,)> for [T; N]
where
    T: DecodeAccounts<A>,
    A: Clone,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: (A,),
    ) -> AnchorResult<Self> {
        try_array_init(|_| T::try_accounts(accounts_context, accounts, arg.0.clone()))
    }

    fn size_hint() -> (usize, Option<usize>) {
        if const { N == 0 } {
            (0, Some(0))
        } else {
            let t_size_hint = T::size_hint();
            (t_size_hint.0 * N, t_size_hint.1.map(|v| v * N))
        }
    }
}
impl<T, const N: usize> DecodeAccounts<()> for [T; N]
where
    T: DecodeAccounts<()>,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: (),
    ) -> AnchorResult<Self> {
        Self::try_accounts(accounts_context, accounts, (arg,))
    }

    fn size_hint() -> (usize, Option<usize>) {
        if const { N == 0 } {
            (0, Some(0))
        } else {
            let t_size_hint = T::size_hint();
            (t_size_hint.0 * N, t_size_hint.1.map(|v| v * N))
        }
    }
}

impl<T, const N: usize, A> ValidateAccounts<[A; N]> for [T; N]
where
    T: ValidateAccounts<A>,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: [A; N]) -> AnchorResult {
        for (t, a) in self.iter_mut().zip(arg) {
            T::validate(t, accounts_context, a)?;
        }
        Ok(())
    }
}
impl<T, const N: usize, A> ValidateAccounts<(A,)> for [T; N]
where
    T: ValidateAccounts<A>,
    A: Clone,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: (A,)) -> AnchorResult {
        for t in self.iter_mut() {
            T::validate(t, accounts_context, arg.0.clone())?;
        }
        Ok(())
    }
}
impl<T, const N: usize> ValidateAccounts<()> for [T; N]
where
    T: ValidateAccounts<()>,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: ()) -> AnchorResult {
        Self::validate(self, accounts_context, (arg,))
    }
}
impl<T, const N: usize, A> CleanupAccounts<[A; N]> for [T; N]
where
    T: CleanupAccounts<A>,
{
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: [A; N]) -> AnchorResult {
        for (t, a) in self.iter_mut().zip(arg) {
            T::cleanup(t, accounts_context, a)?;
        }
        Ok(())
    }
}
impl<T, const N: usize, A> CleanupAccounts<(A,)> for [T; N]
where
    T: CleanupAccounts<A>,
    A: Clone,
{
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: (A,)) -> AnchorResult {
        for t in self.iter_mut() {
            T::cleanup(t, accounts_context, arg.0.clone())?;
        }
        Ok(())
    }
}
impl<T, const N: usize> CleanupAccounts<()> for [T; N]
where
    T: CleanupAccounts<()>,
{
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: ()) -> AnchorResult {
        Self::cleanup(self, accounts_context, (arg,))
    }
}
