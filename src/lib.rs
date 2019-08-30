/// A convenient 0-sized Error type that implements From<Type> and Into<Type: Default>.
/// Useful for when you only really care that "some error happened, and I want to use `?` to handle it"
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AnyError;

impl<T> Into<AnyError> for T {
    fn into(self) -> AnyError {
        AnyError
    }
}

impl<T: Default> From<AnyError> for T {
    fn from(_: AnyError) -> Self {
        Default::default()
    }
}

/// An error that implements From<Type: Debug>
/// Useful for when you still want to keep some error messages, but really want to use `?`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugFmtError(String);

impl<T: Debug> Into<DebugFmtError> for T {
    fn into(self) -> DebugFmtError {
        DebugFmtError(format!("{:?}", self))
    }
}