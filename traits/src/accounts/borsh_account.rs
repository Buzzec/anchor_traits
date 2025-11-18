use crate::error::{AnchorError, AnchorResult};
use crate::traits::account::{Accounts, CleanupAccounts, DecodeAccounts, SingleAccount};
use crate::traits::account_data::AccountData;
use crate::traits::maybe_bool::{MaybeBool, True};
use crate::traits::program::{CurrentProgram, GetProgramId};
use crate::traits::AccountsContext;
use borsh::{BorshDeserialize, BorshSerialize};
use core::marker::PhantomData;
use core::ops::DerefMut;
use derive_more::Deref;
use derive_where::derive_where;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;

#[derive_where(Clone; T: Clone, A: Clone)]
#[derive(Deref)]
pub struct BorshAccount<T, P = CurrentProgram, A = AccountInfo>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount,
    P: GetProgramId,
{
    #[deref]
    data: T,
    account: A,
    pub _program: PhantomData<fn() -> P>,
}
impl<T, P, A> DerefMut for BorshAccount<T, P, A>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount<Mutable = True>,
    P: GetProgramId,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<T, P, A> Accounts for BorshAccount<T, P, A>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount,
    P: GetProgramId,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        A::to_account_metas(&self.account, is_signer)
    }

    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        A::to_account_infos(&self.account)
    }
}
unsafe impl<T, P, A> SingleAccount for BorshAccount<T, P, A>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount,
    P: GetProgramId,
{
    type Mutable = A::Mutable;
    type CanSign = A::CanSign;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        A::account_info_ref(&self.account)
    }
}
impl<T, P, A, Arg> DecodeAccounts<Arg> for BorshAccount<T, P, A>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount + DecodeAccounts<Arg>,
    P: GetProgramId,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: Arg,
    ) -> AnchorResult<Self> {
        let account = A::try_accounts(accounts_context, accounts, arg)?;

        let parse_data = |data_bytes: &[u8]| {
            if data_bytes.len() < T::DISCRIMINANT.len()
                || &data_bytes[..T::DISCRIMINANT.len()] != T::DISCRIMINANT
            {
                Err(AnchorError::InvalidAccountData)
            } else {
                T::try_from_slice(&data_bytes[T::DISCRIMINANT.len()..])
                    .map_err(|_| AnchorError::InvalidAccountData)
            }
        };

        if const { A::Mutable::IS_FALSE } {
            // If readonly we can directly access the data safely.
            let data_bytes = unsafe { account.account_info_ref().borrow_data_unchecked() };
            Ok(Self {
                data: parse_data(data_bytes)?,
                account,
                _program: PhantomData,
            })
        } else {
            let data_bytes = account.account_info_ref().try_borrow_data()?;
            let data = parse_data(&data_bytes)?;
            drop(data_bytes);
            Ok(Self {
                data,
                account,
                _program: PhantomData,
            })
        }
    }
}
impl<T, P, A, Arg> CleanupAccounts<Arg> for BorshAccount<T, P, A>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount + CleanupAccounts<Arg>,
    P: GetProgramId,
{
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: Arg) -> AnchorResult {
        if const { A::Mutable::IS_TRUE } && self.account.account_info_ref().is_writable() {
            borsh::to_writer(
                &mut *self.account_info_ref().try_borrow_mut_data()?,
                &self.data,
            )
            .map_err(|_| AnchorError::InvalidAccountData)?;
        }

        A::cleanup(&mut self.account, accounts_context, arg)
    }
}
