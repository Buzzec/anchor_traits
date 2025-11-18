use crate::error::AnchorResult;
use crate::traits::maybe_bool::MaybeBool;
use crate::traits::AccountsContext;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::pubkey::Pubkey;

pub trait Accounts {
    fn to_account_metas(&self, is_signer: Option<bool>) -> impl Iterator<Item = AccountMeta<'_>>;

    fn to_account_infos(&self) -> impl Iterator<Item = AccountInfo>;
}

/// # Safety
/// `[SingleAccount::Mutable]` must be correct so optimizations can take advantage of
/// mutability/immutability of the data.
pub unsafe trait SingleAccount: Accounts {
    type Mutable: MaybeBool;
    type CanSign: MaybeBool;

    fn account_info_ref(&self) -> &AccountInfo;
    #[inline]
    fn account_info(&self) -> AccountInfo {
        *self.account_info_ref()
    }

    #[inline]
    fn key(&self) -> &Pubkey {
        self.account_info_ref().key()
    }

    #[inline]
    fn owner(&self) -> &Pubkey {
        self.account_info_ref().owner()
    }
}

pub trait DecodeAccounts<A>: Sized + Accounts {
    #[track_caller]
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
    #[track_caller]
    fn validate(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult;
}

pub trait CleanupAccounts<A>: Accounts {
    #[track_caller]
    fn cleanup(&mut self, accounts_context: &mut AccountsContext, arg: A) -> AnchorResult;
}
