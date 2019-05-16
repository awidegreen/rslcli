use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    ResponseError(serde_json::Error),
    InputError(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Network(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ResponseError(err)
    }
}
