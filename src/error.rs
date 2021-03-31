use inkwell::execution_engine::FunctionLookupError;
use inkwell::support::LLVMString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    CompileError(String),
    #[error("{0}")]
    LLVMError(String),
    #[error("{0}")]
    FunctionLookupError(#[from] FunctionLookupError),
}

macro_rules! compile_err {
    ( $($arg:tt)* ) => {
        $crate::Error::CompileError(format!($($arg)*))
    }
}

impl From<LLVMString> for Error {
    fn from(src: LLVMString) -> Self {
        Error::LLVMError(src.to_string())
    }
}
