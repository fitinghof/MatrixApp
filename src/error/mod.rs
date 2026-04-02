use matrix_sdk;
use thiserror;
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error from matrix sdk: {0:?}")]
    MatrixSDK(#[from] matrix_sdk::Error),
    #[error("Matrix sdk parse error: {0:?}")]
    MatrixSDKParse(#[from] matrix_sdk::IdParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
