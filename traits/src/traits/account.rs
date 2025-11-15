use crate::error::AnchorResult;
use crate::traits::AccountsContext;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::pubkey::Pubkey;

pub trait ToAccountMetas {
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>>;
}

pub trait ToAccountInfos {
    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo>;
}

pub trait Accounts: Sized + ToAccountMetas + ToAccountInfos {}

pub trait SingleAccount: Accounts {
    fn account_info_ref(&self) -> &AccountInfo;
    #[inline]
    fn account_info(&self) -> AccountInfo {
        *self.account_info_ref()
    }

    #[inline]
    fn key(&self) -> &Pubkey {
        self.account_info_ref().key()
    }
}

pub trait DecodeAccounts<A>: Accounts {
    fn try_accounts(
        accounts_context: &mut AccountsContext,
        accounts: &mut impl Iterator<Item = AccountInfo>,
        arg: A,
    ) -> AnchorResult<Self>;

    fn size_hint() -> (usize, Option<usize>) {
        (0, None)
    }
}

pub trait ValidateAccounts<A>: Accounts {
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult;
}
