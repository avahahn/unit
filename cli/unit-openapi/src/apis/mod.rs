use http;
use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(hyper::http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

#[derive(Debug)]
pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Body,
}

impl From<(hyper::StatusCode, hyper::body::Body)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Body)) -> Self {
        Error::Api(ApiError { code: e.0, body: e.1 })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        return Error::Http(e);
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

mod request;

mod access_log_api;
pub use self::access_log_api::{AccessLogApi, AccessLogApiClient};
mod applications_api;
pub use self::applications_api::{ApplicationsApi, ApplicationsApiClient};
mod apps_api;
pub use self::apps_api::{AppsApi, AppsApiClient};
mod certificates_api;
pub use self::certificates_api::{CertificatesApi, CertificatesApiClient};
mod config_api;
pub use self::config_api::{ConfigApi, ConfigApiClient};
mod control_api;
pub use self::control_api::{ControlApi, ControlApiClient};
mod listeners_api;
pub use self::listeners_api::{ListenersApi, ListenersApiClient};
mod routes_api;
pub use self::routes_api::{RoutesApi, RoutesApiClient};
mod settings_api;
pub use self::settings_api::{SettingsApi, SettingsApiClient};
mod status_api;
pub use self::status_api::{StatusApi, StatusApiClient};
mod tls_api;
pub use self::tls_api::{TlsApi, TlsApiClient};
mod xff_api;
pub use self::xff_api::{XffApi, XffApiClient};

pub mod client;
pub mod configuration;
mod error;
