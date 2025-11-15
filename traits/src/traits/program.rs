use pinocchio::pubkey::Pubkey;

pub trait ProgramId {
    const ID: Pubkey;
}
