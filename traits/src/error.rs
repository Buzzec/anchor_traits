use pinocchio::program_error::ProgramError;

pub type AnchorError = ProgramError;
pub type AnchorResult<T = ()> = Result<T, AnchorError>;

pub trait CustomErrorGenerator {
    fn generate(self, proposed_error: AnchorError) -> AnchorError;
}

impl CustomErrorGenerator for () {
    fn generate(self, proposed_error: AnchorError) -> AnchorError {
        proposed_error
    }
}

impl CustomErrorGenerator for AnchorError {
    fn generate(self, _proposed_error: AnchorError) -> AnchorError {
        self
    }
}

impl<F> CustomErrorGenerator for F
where
    F: FnOnce(AnchorError) -> AnchorError,
{
    fn generate(self, proposed_error: AnchorError) -> AnchorError {
        self(proposed_error)
    }
}
