use crate::accounts::mutable::ReadOnly;
use crate::error::{AnchorError, AnchorResult};
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::account_data::AccountData;
use crate::traits::constraint::SupportsConstraint;
use crate::traits::maybe_bool::{False, MaybeBool, True};
use crate::traits::program::{CurrentProgram, GetProgramId};
use crate::traits::AccountsContext;
use bytemuck::{CheckedBitPattern, NoUninit};
use core::marker::PhantomData;
use derive_where::derive_where;
use pinocchio::account_info::{AccountInfo, Ref, RefMut};
use pinocchio::instruction::AccountMeta;

#[derive_where(Clone; T: Clone, A: Clone)]
pub struct BytemuckAccount<T, A = ReadOnly<AccountInfo>, P = CurrentProgram>
where
    T: AccountData + CheckedBitPattern,
    A: SingleAccount,
    P: GetProgramId,
{
    account: A,
    _data: PhantomData<fn() -> T>,
    _program: PhantomData<fn() -> P>,
}
impl<T, A, P> BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
    A: SingleAccount,
    P: GetProgramId,
{
    pub fn data(&self) -> AnchorResult<Ref<'_, T>> {
        Ref::try_map(self.account_info_ref().try_borrow_data()?, |data| {
            bytemuck::checked::try_from_bytes(&data[T::DISCRIMINANT.len()..])
        })
        .map_err(|_| AnchorError::InvalidAccountData)
    }

    pub fn data_mut(&mut self) -> AnchorResult<RefMut<'_, T>>
    where
        T: NoUninit,
        A: SingleAccount<Mutable = True>,
    {
        RefMut::try_map(self.account_info_ref().try_borrow_mut_data()?, |data| {
            bytemuck::checked::try_from_bytes_mut(&mut data[T::DISCRIMINANT.len()..])
        })
        .map_err(|_| AnchorError::InvalidAccountData)
    }

    pub fn data_readonly(&self) -> AnchorResult<&'_ T>
    where
        A: SingleAccount<Mutable = False>,
    {
        let data = unsafe { self.account.account_info_ref().borrow_data_unchecked() };
        bytemuck::checked::try_from_bytes(&data[T::DISCRIMINANT.len()..])
            .map_err(|_| AnchorError::InvalidAccountData)
    }
}
impl<T, A, P> Accounts for BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
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
unsafe impl<T, A, P> SingleAccount for BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
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
impl<T, A, P, Arg> DecodeAccounts<Arg> for BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
    A: SingleAccount + DecodeAccounts<Arg>,
    P: GetProgramId,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: Arg,
    ) -> AnchorResult<Self> {
        Ok(Self {
            account: A::try_accounts(accounts_context, accounts, arg)?,
            _data: PhantomData,
            _program: PhantomData,
        })
    }

    #[inline]
    fn size_hint() -> (usize, Option<usize>) {
        A::size_hint()
    }
}
impl<T, A, P, Arg> ValidateAccounts<Arg> for BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
    A: SingleAccount + ValidateAccounts<Arg>,
    P: GetProgramId,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: Arg) -> AnchorResult {
        if self.account.owner() != P::program_id(accounts_context) {
            return Err(AnchorError::InvalidAccountOwner);
        }

        let check_discriminant = |data: &[u8]| {
            if data.len() < T::DISCRIMINANT.len()
                || &data[..T::DISCRIMINANT.len()] != T::DISCRIMINANT
            {
                Err(AnchorError::InvalidAccountData)
            } else {
                Ok(())
            }
        };

        if const { A::Mutable::IS_FALSE } {
            // If readonly we can directly access the data safely.
            check_discriminant(unsafe { self.account.account_info_ref().borrow_data_unchecked() })?;
        } else {
            check_discriminant(&self.account.account_info_ref().try_borrow_data()?)?;
        }

        A::validate(&mut self.account, accounts_context, arg)
    }
}
impl<T, A, P, Arg> CleanupAccounts<Arg> for BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
    A: SingleAccount + CleanupAccounts<Arg>,
    P: GetProgramId,
{
    #[inline]
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: Arg) -> AnchorResult {
        A::cleanup(&mut self.account, accounts_context, arg)
    }
}
impl<T, A, P, C> SupportsConstraint<C> for BytemuckAccount<T, A, P>
where
    T: AccountData + CheckedBitPattern,
    A: SingleAccount + SupportsConstraint<C>,
    P: GetProgramId,
{
    fn early_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        A::early_validation(&mut self.account, constraint, context)
    }

    fn late_validation(
        &mut self,
        constraint: &mut C,
        context: &mut AccountsContext,
    ) -> AnchorResult {
        A::late_validation(&mut self.account, constraint, context)
    }

    fn cleanup(&mut self, constraint: &mut C, context: &mut AccountsContext) -> AnchorResult {
        A::cleanup(&mut self.account, constraint, context)
    }
}
