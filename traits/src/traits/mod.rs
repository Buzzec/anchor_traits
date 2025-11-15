use pinocchio::pubkey::Pubkey;

pub mod account;
pub mod builder;
pub mod constraint;
pub mod program;
pub mod seeds;

#[non_exhaustive]
pub struct AccountsContext<'a> {
    pub current_program_id: &'a Pubkey,
}
