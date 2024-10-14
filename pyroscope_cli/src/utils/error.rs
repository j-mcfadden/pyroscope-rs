/// Result Alias with BackendError
pub type Result<T> = std::result::Result<T, Error>;

/// Error type of PyroscopeBackend
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Other: {}", &.0)]
    AdHoc(String),

    #[error("{msg}: {source:?}")]
    Compat {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Clap(#[from] clap::Error),

    #[error(transparent)]
    SetLogger(#[from] log::SetLoggerError),

    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    PyroscopeError(#[from] pyroscope::PyroscopeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Error {
    /// Create a new instance of PyroscopeError
    pub fn new(msg: &str) -> Self {
        Error::AdHoc(msg.to_string())
    }

    /// Create a new instance of PyroscopeError with source
    pub fn new_with_source<E>(msg: &str, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Error::Compat {
            msg: msg.to_string(),
            source: Box::new(source),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Error::AdHoc("Poison Error".to_owned())
    }
}
