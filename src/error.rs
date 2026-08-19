use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error {:?}", .0)]
    IoError(#[from] std::io::Error),

    #[error("URL Parse Error {:?}", .0)]
    UrlError(#[from] url::ParseError),

    #[error("POST callback error {}", .0)]
    PostCallbackError(String),

    #[error("POST callback invalid / missing state")]
    PostCallbackMissingState,

    #[error("POST callback invalid / missing code")]
    PostCallbackMissingCode,

    #[error("AsyncChannelRecvError")]
    AsyncChannelRecvError(#[from] async_channel::RecvError),

    #[error("AsyncChannelSendError")]
    AsyncChannelSendError,

    #[error("Tokio join error: {:?}", .0)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error("Webserver shutdown prior to receiving token")]
    WebServerPrematureShutdown,

    #[error("CSRF-prevention state check failed")]
    StateMismatchViolation,

    #[error("Temporary token post failed, could not retrieve permanent token")]
    TemporaryTokenPostFailed,

    #[error("Reqwest error: {:?}", .0)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Serde json error: {:?}", .0)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Time error: {:?}", .0)]
    StdTimeError(#[from] std::time::SystemTimeError),
}

pub type Res<T> = Result<T, Error>;

impl<T> From<async_channel::SendError<T>> for Error {
    fn from(_: async_channel::SendError<T>) -> Error {
        Error::AsyncChannelSendError
    }
}
