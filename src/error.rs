use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Network(hyper::Error),
    ResponseError(serde_json::Error),
    InputError(String),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Network(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ResponseError(err)
    }
}

