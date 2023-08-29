use std::collections::HashMap;
use std::error;
use std::fmt;

use base64ct::{Base64, Encoding};

pub(crate) fn get_header_map(response: &ureq::Response) -> HashMap<String, String> {
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
    pub content: String,
    pub entity: T,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Error<T> {
    Ureq(ureq::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Ureq(e) => ("ureq", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Ureq(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
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
    format!("Basic {}", Base64::encode_string(string.as_bytes()))
}

pub mod default_api;

pub mod configuration;
