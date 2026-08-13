use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    
}

pub type Res<T> = Result<T, Error>;
