use crate::error::AnchorResult;
use crate::traits::account::{
    Accounts, DecodeAccounts, ToAccountInfos, ToAccountMetas, ValidateAccounts,
};
use crate::traits::AccountsContext;
use core::iter::once;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

impl ToAccountMetas for AccountInfo {
    #[inline]
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>> {
        once(AccountMeta::new(
            self.key(),
            is_signer.unwrap_or_else(|| self.is_signer()),
            self.is_writable(),
        ))
    }
}
impl ToAccountInfos for AccountInfo {
    #[inline]
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo> {
        once(*self)
    }
}
impl Accounts for AccountInfo {}
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
