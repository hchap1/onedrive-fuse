use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error {:?}", .0)]
    IoError(#[from] std::io::Error)
}

pub type Res<T> = Result<T, Error>;
