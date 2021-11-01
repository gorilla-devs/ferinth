use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("A HTTP(S) error occured {0}")]
    HTTPError(#[from] reqwest::Error),
}
