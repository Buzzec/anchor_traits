use crate::traits::program::ProgramId;
use pinocchio::pubkey::Pubkey;

pub struct System;
impl ProgramId for System {
    const ID: Pubkey = [0; 32];
}
