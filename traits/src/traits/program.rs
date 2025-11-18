use crate::traits::maybe_bool::{Bool, True};
use crate::traits::AccountsContext;
use pinocchio::pubkey::Pubkey;

pub trait ProgramId {
    const ID: Pubkey;

    type IsCurrentProgram: Bool;
}
pub trait GetProgramId {
    type IsCurrentProgram: Bool;

    fn program_id<'a>(accounts_context: &AccountsContext<'a>) -> &'a Pubkey;
}
impl<P> GetProgramId for P
where
    P: ProgramId,
{
    type IsCurrentProgram = <P as ProgramId>::IsCurrentProgram;

    fn program_id<'a>(_accounts_context: &AccountsContext<'a>) -> &'a Pubkey {
        &Self::ID
    }
}

pub struct CurrentProgram;
impl GetProgramId for CurrentProgram {
    type IsCurrentProgram = True;

    fn program_id<'a>(accounts_context: &AccountsContext<'a>) -> &'a Pubkey {
        accounts_context.current_program_id
    }
}
