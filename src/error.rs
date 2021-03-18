use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    CompileError(String),
}

macro_rules! compile_err {
    ( $($arg:tt)* ) => {
        $crate::Error::CompileError(format!($($arg)*))
    }
}
