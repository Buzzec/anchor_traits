use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, CleanupAccounts, DecodeAccounts, SingleAccount, ValidateAccounts,
};
use crate::traits::maybe_bool::Unknown;
use crate::traits::AccountsContext;
use core::iter::once;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

impl Accounts for AccountInfo {
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        once(AccountMeta::new(
            self.key(),
            is_signer.unwrap_or_else(|| self.is_signer()),
            self.is_writable(),
        ))
    }

    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        once(*self)
    }
}
unsafe impl SingleAccount for AccountInfo {
    type Mutable = Unknown;
    type CanSign = Unknown;

    #[inline]
    fn account_info_ref(&self) -> &AccountInfo {
        self
    }
}
impl DecodeAccounts<()> for AccountInfo {
    fn try_accounts(
        _accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        _arg: (),
    ) -> AnchorResult<Self> {
        accounts.next().ok_or(ProgramError::NotEnoughAccountKeys)
    }
}
impl ValidateAccounts<()> for AccountInfo {
    fn validate(&mut self, _accounts_context: &mut AccountsContext, _arg: ()) -> AnchorResult {
        Ok(())
    }
}
impl CleanupAccounts<()> for AccountInfo {
    fn cleanup(&mut self, _accounts_context: &mut AccountsContext, _arg: ()) -> AnchorResult {
        Ok(())
    }
}
