use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetaError {
    #[error("400 bad request")]
    BadRequest,
    #[error("401 unauthorized")]
    Unauthorized,
    #[error("404 not found")]
    NotFound,
    #[error("409 conflict")]
    Conflict,
    #[error("HTTP error: {status_code} {msg}")]
    HTTPError { status_code: u16, msg: String },
    #[error("transport error")]
    TransportError,
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("JSON error")]
    JSONError(#[from] serde_json::Error),
}

impl From<ureq::Error> for DetaError {
    fn from(ureq_err: ureq::Error) -> Self {
        match ureq_err {
            ureq::Error::Status(400, _) => DetaError::BadRequest,
            ureq::Error::Status(401, _) => DetaError::Unauthorized,
            ureq::Error::Status(404, _) => DetaError::NotFound,
            ureq::Error::Status(409, _) => DetaError::Conflict,
            ureq::Error::Status(status_code, res) => DetaError::HTTPError {
                status_code,
                msg: res.status_text().to_string(),
            },
            ureq::Error::Transport(_) => DetaError::TransportError,
        }
    }
}
