use pinocchio::pubkey::Pubkey;

pub mod account;
pub mod account_data;
pub mod builder;
pub mod constraint;
pub mod maybe_bool;
pub mod program;
pub mod seeds;

#[non_exhaustive]
pub struct AccountsContext<'a> {
    pub current_program_id: &'a Pubkey,
}
