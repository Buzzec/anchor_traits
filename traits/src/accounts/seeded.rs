use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::maybe_bool::{MaybeBool, Or};
use crate::traits::program::{CurrentProgram, GetProgramId};
use crate::traits::seeds::{SeededAccount, Seeds};
use crate::traits::AccountsContext;
use core::marker::PhantomData;
use core::ops::BitOr;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::{create_program_address, Pubkey};

#[derive(Copy, Clone, Debug)]
pub struct SeedsWithBump<S> {
    pub seeds: S,
    pub bump: u8,
}
impl<S> SeedsWithBump<S> {
    pub fn create_program_address(&self, program_id: &Pubkey) -> AnchorResult<Pubkey>
    where
        S: Seeds,
    {
        self.seeds
            .with_seeds_and_bump(self.bump, |seeds| create_program_address(seeds, program_id))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Seeded<T, S = <T as SeededAccount>::Seeds, P = CurrentProgram> {
    pub account: T,
    pub seeds: Option<SeedsWithBump<S>>,
    pub _phantom_program: PhantomData<fn() -> P>,
}
impl<T, S, P> Accounts for Seeded<T, S, P>
where
    T: Accounts,
{
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        T::to_account_metas(&self.account, is_signer)
    }

    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        T::to_account_infos(&self.account)
    }
}
unsafe impl<T, S, P> SingleAccount for Seeded<T, S, P>
where
    T: SingleAccount,
    P: GetProgramId,
    P::IsCurrentProgram: BitOr<T::CanSign>,
    Or<P::IsCurrentProgram, T::CanSign>: MaybeBool,
{
    type Mutable = T::Mutable;
    type CanSign = Or<P::IsCurrentProgram, T::CanSign>;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        T::account_info_ref(&self.account)
    }
}
impl<T, S, P, A> DecodeAccounts<A> for Seeded<T, S, P>
where
    T: DecodeAccounts<A>,
{
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: A,
    ) -> AnchorResult<Self> {
        Ok(Self {
            account: T::try_accounts(accounts_context, accounts, arg)?,
            seeds: None,
            _phantom_program: PhantomData,
        })
    }

    #[inline]
    fn size_hint() -> (usize, Option<usize>) {
        T::size_hint()
    }
}
impl<T, S, P> ValidateAccounts<S> for Seeded<T, S, P>
where
    T: SingleAccount + ValidateAccounts<()>,
    S: Seeds,
    P: GetProgramId,
    P::IsCurrentProgram: BitOr<T::CanSign>,
    Or<P::IsCurrentProgram, T::CanSign>: MaybeBool,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: S) -> AnchorResult {
        Self::validate(self, accounts_context, (arg, ()))
    }
}
impl<T, S, P> ValidateAccounts<SeedsWithBump<S>> for Seeded<T, S, P>
where
    T: SingleAccount + ValidateAccounts<()>,
    S: Seeds,
    P: GetProgramId,
    P::IsCurrentProgram: BitOr<T::CanSign>,
    Or<P::IsCurrentProgram, T::CanSign>: MaybeBool,
{
    fn validate(
        &mut self,
        accounts_context: &mut AccountsContext,
        arg: SeedsWithBump<S>,
    ) -> AnchorResult {
        Self::validate(self, accounts_context, (arg, ()))
    }
}
impl<T, S, P, A> ValidateAccounts<(S, A)> for Seeded<T, S, P>
where
    T: SingleAccount + ValidateAccounts<A>,
    S: Seeds,
    P: GetProgramId,
    P::IsCurrentProgram: BitOr<T::CanSign>,
    Or<P::IsCurrentProgram, T::CanSign>: MaybeBool,
{
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: (S, A)) -> AnchorResult {
        let (found_key, bump) = arg.0.find_program_address(P::program_id(accounts_context));

        if found_key == *self.key() {
            self.seeds = Some(SeedsWithBump { seeds: arg.0, bump });

            T::validate(&mut self.account, accounts_context, arg.1)
        } else {
            Err(ProgramError::InvalidSeeds)
        }
    }
}
impl<T, S, P, A> ValidateAccounts<(SeedsWithBump<S>, A)> for Seeded<T, S, P>
where
    T: SingleAccount + ValidateAccounts<A>,
    S: Seeds,
    P: GetProgramId,
    P::IsCurrentProgram: BitOr<T::CanSign>,
    Or<P::IsCurrentProgram, T::CanSign>: MaybeBool,
{
    fn validate(
        &mut self,
        accounts_context: &mut AccountsContext,
        arg: (SeedsWithBump<S>, A),
    ) -> AnchorResult {
        if self.key()
            == &arg
                .0
                .create_program_address(P::program_id(accounts_context))?
        {
            self.seeds = Some(arg.0);

            T::validate(&mut self.account, accounts_context, arg.1)
        } else {
            Err(ProgramError::InvalidSeeds)
        }
    }
}
impl<T, S, P> ValidateAccounts<()> for Seeded<T, S, P>
where
    T: SingleAccount + ValidateAccounts<()>,
    S: Seeds + Default,
    P: GetProgramId,
    P::IsCurrentProgram: BitOr<T::CanSign>,
    Or<P::IsCurrentProgram, T::CanSign>: MaybeBool,
{
    #[inline]
    fn validate(&mut self, accounts_context: &mut AccountsContext, _arg: ()) -> AnchorResult {
        Self::validate(self, accounts_context, S::default())
    }
}
impl<T, S, P> CleanupAccounts<()> for Seeded<T, S, P>
where
    T: CleanupAccounts<()>,
{
    #[inline]
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: ()) -> AnchorResult {
        T::cleanup(&mut self.account, accounts_context, arg)
    }
}
