use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetaError {
    #[error("404 not found")]
    NotFound,
    #[error("HTTP error: {status_code} {msg}")]
    HTTPError { status_code: u16, msg: String },
    #[error("transport error")]
    TransportError,
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

impl From<ureq::Error> for DetaError {
    fn from(ureq_err: ureq::Error) -> Self {
        match ureq_err {
            ureq::Error::Status(404, _) => DetaError::NotFound,
            ureq::Error::Status(status_code, res) => DetaError::HTTPError {
                status_code,
                msg: res.status_text().to_string(),
            },
            ureq::Error::Transport(_) => DetaError::TransportError,
        }
    }
}
