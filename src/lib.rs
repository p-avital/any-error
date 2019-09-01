#![feature(optin_builtin_traits)]
use core::any::Any;

/// A convenient 0-sized Error type that implements From<Type> and Into<Type: Default>.
/// Useful for when you only really care that "some error happened, and I want to use `?` to handle it"
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SomeError;
pub auto trait NotSomeError {}
impl !NotSomeError for SomeError {}

impl Default for SomeError {
    fn default() -> Self {
        SomeError
    }
}

impl<T: NotSomeError> From<T> for SomeError {
    fn from(_: T) -> SomeError {
        SomeError
    }
}

/// An error that implements From<Type: Debug>
/// Useful for when you still want to keep some error messages, but really want to use `?`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormattedError(pub String);
pub auto trait NotFormattedError {}
impl !NotFormattedError for FormattedError {}

impl FormattedError {
    pub fn new(string: &str) -> Self {
        FormattedError(string.to_owned())
    }
}

impl Default for FormattedError {
    fn default() -> Self {
        FormattedError::new("Default FormattedError")
    }
}

impl<T: core::fmt::Debug + NotFormattedError> From<T> for FormattedError {
    fn from(t: T) -> FormattedError {
        FormattedError(format!("{:?}", t))
    }
}

/// When you may want to return various error types,
/// but are too lazy to use an enum and implement From for Everything.
pub struct AnyError(pub Box<dyn Any>);
pub auto trait NotAnyError {}
impl !NotAnyError for AnyError {}

impl Default for AnyError {
    fn default() -> Self {
        ().into()
    }
}

impl<T: NotAnyError + Any> From<T> for AnyError {
    fn from(t: T) -> AnyError {
        AnyError(Box::new(t) as Box<dyn Any>)
    }
}

impl AnyError {
    pub fn downcast<T: Any>(self) -> Result<T, Self> {
        match self.0.downcast::<T>() {
            Ok(t) => Ok(*t),
            Err(e) => Err(AnyError(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{AnyError, FormattedError, SomeError};

    struct FormatableError;
    impl core::fmt::Debug for FormatableError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "FormatableErrorDebug")
        }
    }

    fn unit_or_formatable_error(ok: bool) -> Result<(), FormatableError> {
        match ok {
            true => Ok(()),
            false => Err(FormatableError),
        }
    }

    #[test]
    fn some_test() {
        let test = |ok| -> Result<(), SomeError> { Ok(unit_or_formatable_error(ok)?) };
        assert_eq!(test(true), Ok(()));
        assert_eq!(test(false), Err(SomeError));
    }

    #[test]
    fn formatted_test() {
        let test = |ok| -> Result<(), FormattedError> { Ok(unit_or_formatable_error(ok)?) };
        assert_eq!(test(true), Ok(()));
        assert_eq!(
            test(false),
            Err(FormattedError("FormatableErrorDebug".to_owned()))
        );
    }

    #[test]
    fn any_test() {
        let test = |ok| -> Result<(), AnyError> { Ok(unit_or_formatable_error(ok)?) };
        assert!(test(true).is_ok());
        match test(false) {
            Ok(_) => panic!("test(false) should never be Ok(_)"),
            Err(error) => match error.0.downcast::<FormatableError>() {
                Ok(downcasted) => println!("Successfully Downcasted: {:?}", downcasted),
                Err(_) => panic!("Couldn't downcast after boxing in AnyError"),
            },
        }
    }
}
