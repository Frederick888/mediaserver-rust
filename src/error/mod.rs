use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HandlerError {
    StringError(String),
    MissingQueryParam(String),
    IOError(String),
    AuthError(Option<String>, Option<String>),
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HandlerError::StringError(ref i) | HandlerError::IOError(ref i) => {
                fmt::Display::fmt(i, f)
            }
            HandlerError::MissingQueryParam(ref i) => {
                fmt::Display::fmt(&format!("Missing query parameter: {}", i), f)
            }
            HandlerError::AuthError(ref u, ref p) => {
                fmt::Display::fmt(
                    &format!(
                        "Authentication error with username: {}, password: {}",
                        u.clone().unwrap_or_else(|| "NULL".to_owned()),
                        p.clone().unwrap_or_else(|| "NULL".to_owned())
                    ),
                    f,
                )
            }
        }
    }
}

impl Error for HandlerError {
    fn description(&self) -> &str {
        match *self {
            HandlerError::StringError(ref i) | HandlerError::IOError(ref i) => i,
            HandlerError::MissingQueryParam(_) => "Missing query parameter",
            HandlerError::AuthError(_, _) => "Authentication error",
        }
    }
}

impl From<HandlerError> for ::iron::IronError {
    fn from(e: HandlerError) -> ::iron::IronError {
        match e {
            HandlerError::IOError(_) => {
                ::iron::IronError::new(e, ::iron::status::InternalServerError)
            }
            _ => ::iron::IronError::new(e, ::iron::status::BadRequest),
        }
    }
}

impl From<::std::io::Error> for HandlerError {
    fn from(e: ::std::io::Error) -> HandlerError {
        HandlerError::IOError(e.to_string())
    }
}
