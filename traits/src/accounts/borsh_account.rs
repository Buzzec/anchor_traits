use crate::accounts::mutable::ReadOnly;
use crate::error::{AnchorError, AnchorResult};
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::account_data::AccountData;
use crate::traits::constraint::SupportsConstraint;
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
pub struct BorshAccount<T, A = ReadOnly<AccountInfo>, P = CurrentProgram>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount,
    P: GetProgramId,
{
    #[deref]
    data: T,
    account: A,
    _program: PhantomData<fn() -> P>,
}
impl<T, A, P> DerefMut for BorshAccount<T, A, P>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount<Mutable = True>,
    P: GetProgramId,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<T, A, P> Accounts for BorshAccount<T, A, P>
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
unsafe impl<T, A, P> SingleAccount for BorshAccount<T, A, P>
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
impl<T, A, P, Arg> DecodeAccounts<Arg> for BorshAccount<T, A, P>
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
impl<T, A, P, Arg> ValidateAccounts<Arg> for BorshAccount<T, A, P>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount + ValidateAccounts<Arg>,
    P: GetProgramId,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: Arg) -> AnchorResult {
        if self.account.owner() != P::program_id(accounts_context) {
            return Err(AnchorError::InvalidAccountOwner);
        }
        A::validate(&mut self.account, accounts_context, arg)
    }
}
impl<T, A, P, Arg> CleanupAccounts<Arg> for BorshAccount<T, A, P>
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
impl<T, A, P, C> SupportsConstraint<C> for BorshAccount<T, A, P>
where
    T: AccountData + BorshSerialize + BorshDeserialize,
    A: SingleAccount + SupportsConstraint<C>,
    P: GetProgramId,
{
    #[inline]
    fn early_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        A::early_validation(&mut self.account, constraint, context)
    }

    #[inline]
    fn late_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        A::late_validation(&mut self.account, constraint, context)
    }

    #[inline]
    fn cleanup(&mut self, constraint: &mut C, context: &mut AccountsContext) -> AnchorResult {
        A::cleanup(&mut self.account, constraint, context)
    }
}
