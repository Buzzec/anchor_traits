pub trait Builder {
    type Output;
}

pub trait BuilderStart: Builder {
    fn new() -> Self;
}

pub trait BuilderField<F>: Builder {
    type FieldType;
    type AfterSet: Builder<Output = Self::Output>;

    fn set_field(self, field: Self::FieldType) -> Self::AfterSet;
}

#[diagnostic::on_unimplemented(
    message = "Missing required fields for constraint",
    label = "Constraint missing required fields",
    note = "Look at documentation of constraint for required fields."
)]
pub trait BuilderFinish: Builder {
    fn finish(self) -> Self::Output;
}
