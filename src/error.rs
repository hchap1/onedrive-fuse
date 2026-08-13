use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error {:?}", .0)]
    IoError(#[from] std::io::Error),

    #[error("URL Parse Error {:?}", .00)]
    UrlError(#[from] url::ParseError),
}

pub type Res<T> = Result<T, Error>;
