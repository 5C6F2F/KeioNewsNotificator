use crate::config::Config;
use std::process::exit;

pub trait RecordError {
    fn record(&self)
    where
        Self: std::fmt::Debug,
    {
        let message = "\nエラーが起きました\n確認してください".to_string();
        match Config::new().line.send(message) {
            Ok(_) => exit(0),
            Err(_) => exit(1),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NewNewsError(NewNewsError),
    OldNewsError(OldNewsError),
    NotifyOnLineError(NotifyOnLineError),
}

impl RecordError for Error {}

impl From<NewNewsError> for Error {
    fn from(err: NewNewsError) -> Self {
        Self::NewNewsError(err)
    }
}

impl From<OldNewsError> for Error {
    fn from(err: OldNewsError) -> Self {
        Self::OldNewsError(err)
    }
}

impl From<NotifyOnLineError> for Error {
    fn from(err: NotifyOnLineError) -> Self {
        Self::NotifyOnLineError(err)
    }
}

#[derive(Debug)]
pub enum NewNewsError {
    UnsuccessRequest(reqwest::Error),
    FailureUpdateOldNewsN(std::io::Error),
}

impl RecordError for NewNewsError {}

impl From<reqwest::Error> for NewNewsError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnsuccessRequest(err)
    }
}

impl From<std::io::Error> for NewNewsError {
    fn from(err: std::io::Error) -> Self {
        Self::FailureUpdateOldNewsN(err)
    }
}

#[derive(Debug)]
pub enum OldNewsError {
    FailureLoadOldNews(std::io::Error),
}

impl RecordError for OldNewsError {}

impl From<std::io::Error> for OldNewsError {
    fn from(err: std::io::Error) -> Self {
        Self::FailureLoadOldNews(err)
    }
}

#[derive(Debug)]
pub enum NotifyOnLineError {
    InvalidToKen(reqwest::header::InvalidHeaderValue),
    UnsuccessRequest(reqwest::Error),
    UnsuccessResponse(reqwest::blocking::Response),
}

impl RecordError for NotifyOnLineError {}

impl From<reqwest::header::InvalidHeaderValue> for NotifyOnLineError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidToKen(err)
    }
}

impl From<reqwest::Error> for NotifyOnLineError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnsuccessRequest(err)
    }
}

impl RecordError for std::io::Error {}
