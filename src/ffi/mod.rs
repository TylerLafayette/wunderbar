pub use core_graphics::geometry::{CGPoint, CGRect, CGSize};

pub mod core_services;
pub mod sls;

pub type CGResult<T> = Result<T, CGError>;

use thiserror::Error;

/// A uniform type for result codes returned by functions in Core Graphics.
///
/// Original documentation:
/// https://developer.apple.com/documentation/coregraphics/cgerror
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CGError {
    /// The requested operation is inappropriate for the parameters passed in, or the current
    /// system state.
    #[error("cannot complete the requested operation")]
    CannotComplete,

    /// A general failure occurred.
    #[error("a general failure occurred")]
    Failure,

    /// One or more of the parameters passed to a function are invalid. Check for NULL pointers.
    #[error(
        "one or more of the parameters passed to a function are invalid. check for null pointers"
    )]
    IllegalArgument,

    /// The parameter representing a connection to the window server is invalid.
    #[error("the connection parameter is invalid")]
    InvalidConnection,

    /// The CPSProcessSerNum or context identifier parameter is not valid.
    #[error("the CPSProcessSerNum or context identifier parameter is not valid")]
    InvalidContext,

    /// The requested operation is not valid for the parameters passed in, or the current system
    /// state.
    #[error("the requested operation is not valid")]
    InvalidOperation,

    /// The requested operation could not be completed as the indicated resources were not found.
    #[error(
        "the requested operation could not be completed as the indicated resources were not found"
    )]
    NoneAvailable,

    /// Return value from obsolete function stubs present for binary compatibility, but not typically called.
    #[error("not implemented")]
    NotImplemented,

    /// A parameter passed in has a value that is inappropriate, or which does not map to a useful
    /// operation or value.
    #[error("not implemented")]
    RangeCheck,

    /// The requested operation was completed successfully.
    ///
    /// NOTE: This value will not be returned by this library's API.
    #[error("success (this value should not have been returned lol)")]
    Success,

    /// A data type or token was encountered that did not match the expected type or token.
    #[error("a data type or token was encountered that did not match expected types (type checker failed)")]
    TypeCheck,
}

impl From<i32> for CGError {
    fn from(value: i32) -> Self {
        // Error codes referenced from
        // https://developer.apple.com/documentation/coregraphics/cgerror
        match value {
            1004 => Self::CannotComplete,
            1000 => Self::Failure,
            1001 => Self::IllegalArgument,
            1002 => Self::InvalidConnection,
            1003 => Self::InvalidContext,
            1010 => Self::InvalidOperation,
            1011 => Self::NoneAvailable,
            1006 => Self::NotImplemented,
            1007 => Self::RangeCheck,
            0 => Self::Success,
            1008 => Self::TypeCheck,
            _ => panic!(
                "CGError returned by Apple framework contains invalid/unexpected value `{}`",
                value
            ),
        }
    }
}

impl CGError {
    /// For a value that implements [`Into<CGError>`], this function will convert the value into
    /// a [`CGError`] and return [`Ok(())`] if it is a [`CGError::Success`]. Otherwise, it will
    /// return the [`CGError`] wrapped in an [`Err`].
    ///
    /// NOTE: This function will panic if the value can not be parsed into a valid i[`CGError`].
    pub(super) fn result_from(value: impl Into<Self>) -> Result<(), Self> {
        let err = value.into();
        if let Self::Success = err {
            Ok(())
        } else {
            Err(err)
        }
    }
}
