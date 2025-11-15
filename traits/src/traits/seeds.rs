use crate::traits::program::ProgramId;
use crate::traits::AccountsContext;
use alloc::vec::Vec;
use pinocchio::pubkey::{find_program_address, Pubkey};

pub trait Seeds {
    fn with_seeds<O>(&self, f: impl FnOnce(&[&[u8]]) -> O) -> O;
    fn with_seeds_and_bump<O>(&self, bump: u8, f: impl FnOnce(&[&[u8]]) -> O) -> O;
    fn seeds(&self) -> Vec<&[u8]>;
    fn find_program_address(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        self.with_seeds(|seeds| find_program_address(seeds, program_id))
    }
}
pub trait SeededAccount {
    type Seeds: Seeds;
}
pub trait SeedProgram {
    fn program_id<'a>(accounts_context: &AccountsContext<'a>) -> &'a Pubkey;
}
impl<P> SeedProgram for P
where
    P: ProgramId,
{
    fn program_id<'a>(_accounts_context: &AccountsContext<'a>) -> &'a Pubkey {
        &Self::ID
    }
}
