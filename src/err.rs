use std::fmt;

pub type Result<T> = std::result::Result<T, JellyfinError>;

#[derive(Debug)]
pub enum JellyfinError {
    NetworkError(surf::Error),
    UrlParseError(url::ParseError),
    StrFmtError(strfmt::FmtError),
    AuthNotFound
}

impl fmt::Display for JellyfinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(v) => {
                write!(f,"{}", v)
            }
            Self::UrlParseError(v) => {
                write!(f,"{}", v)
            }
            Self::StrFmtError(v) => {
                write!(f,"{}", v)
            }
            Self::AuthNotFound => {
                write!(f, "Unauthorized.")
            }
        }
    }
}

impl From<surf::Error> for JellyfinError {
    fn from(value: surf::Error) -> Self {
        Self::NetworkError(value)
    }
}

impl From<url::ParseError> for JellyfinError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}

impl From<strfmt::FmtError> for JellyfinError {
    fn from(value: strfmt::FmtError) -> Self {
        Self::StrFmtError(value)
    }
}