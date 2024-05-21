use std::collections::HashMap;
use std::error;
use std::fmt;

use base64::{engine::general_purpose, Engine};
use serde::de::DeserializeOwned;

fn get_header_map(response: &ureq::Response) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    let names = response.headers_names();
    for name in names {
        if let Some(value) = response.header(&name) {
            headers.insert(name, value.to_string());
        }
    }

    headers
}

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: u16,
    pub content: Vec<u8>,
    pub entity: T,
    pub headers: HashMap<String, String>,
}

impl<T> ResponseContent<T> {
    fn new<F, E>(response: ureq::Response, f: F) -> Result<Self, Error<E>>
    where
        F: FnOnce(&[u8]) -> Result<T, Error<E>>,
    {
        let status = response.status();
        let headers = get_header_map(&response);
        let mut content = Vec::new();
        response.into_reader().read_to_end(&mut content)?;
        let entity = f(&content)?;
        Ok(Self {
            status,
            content,
            entity,
            headers,
        })
    }
}

impl ResponseContent<()> {
    fn unit<E>(response: ureq::Response) -> Result<Self, Error<E>> {
        Self::new(response, |_| Ok(()))
    }
}

impl ResponseContent<Vec<u8>> {
    fn bytes<E>(response: ureq::Response) -> Result<Self, Error<E>> {
        Self::new(response, |content| Ok(content.into()))
    }
}

impl ResponseContent<String> {
    fn string<E>(response: ureq::Response) -> Result<Self, Error<E>> {
        Self::new(response, |content| {
            String::from_utf8(content.into()).map_err(From::from)
        })
    }
}

impl<T: DeserializeOwned> ResponseContent<T> {
    fn deserialized<E>(response: ureq::Response) -> Result<Self, Error<E>> {
        Self::new(response, |content| {
            serde_json::from_slice(content).map_err(From::from)
        })
    }
}

#[derive(Debug)]
pub enum Error<T> {
    Multipart {
        field: Option<String>,
        error: std::io::Error,
    },
    Ureq(ureq::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
    StringParse(std::string::FromUtf8Error),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Multipart { field, error } => {
                let error = match field {
                    Some(field) => format!("failed to encode {field}: {error}"),
                    None => error.to_string(),
                };
                ("multipart", error)
            }
            Error::Ureq(e) => ("ureq", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
            Error::StringParse(e) => ("string", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Multipart { error, .. } => error,
            Error::Ureq(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
            Error::StringParse(e) => e,
        })
    }
}

impl<T> From<multipart::client::lazy::LazyError<'_, std::io::Error>> for Error<T> {
    fn from(e: multipart::client::lazy::LazyError<'_, std::io::Error>) -> Self {
        Self::Multipart {
            field: e.field_name.map(|s| s.into_owned()),
            error: e.error,
        }
    }
}

impl<T> From<ureq::Error> for Error<T> {
    fn from(e: ureq::Error) -> Self {
        Error::Ureq(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl<T> From<std::string::FromUtf8Error> for Error<T> {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::StringParse(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub(crate) fn basic_auth(auth: &configuration::BasicAuth) -> String {
    let string = format!("{}:{}", auth.0, auth.1.as_ref().unwrap_or(&"".to_string()));
    format!("Basic {}", general_purpose::STANDARD.encode(string)).to_string()
}

pub mod default_api;

pub mod configuration;
