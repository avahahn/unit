use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    access_log_api: Box<dyn crate::apis::AccessLogApi>,
    applications_api: Box<dyn crate::apis::ApplicationsApi>,
    apps_api: Box<dyn crate::apis::AppsApi>,
    certificates_api: Box<dyn crate::apis::CertificatesApi>,
    config_api: Box<dyn crate::apis::ConfigApi>,
    control_api: Box<dyn crate::apis::ControlApi>,
    listeners_api: Box<dyn crate::apis::ListenersApi>,
    routes_api: Box<dyn crate::apis::RoutesApi>,
    settings_api: Box<dyn crate::apis::SettingsApi>,
    status_api: Box<dyn crate::apis::StatusApi>,
    tls_api: Box<dyn crate::apis::TlsApi>,
    xff_api: Box<dyn crate::apis::XffApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
    where
        C: Clone + std::marker::Send + Sync + 'static,
    {
        let rc = Rc::new(configuration);

        APIClient {
            access_log_api: Box::new(crate::apis::AccessLogApiClient::new(rc.clone())),
            applications_api: Box::new(crate::apis::ApplicationsApiClient::new(rc.clone())),
            apps_api: Box::new(crate::apis::AppsApiClient::new(rc.clone())),
            certificates_api: Box::new(crate::apis::CertificatesApiClient::new(rc.clone())),
            config_api: Box::new(crate::apis::ConfigApiClient::new(rc.clone())),
            control_api: Box::new(crate::apis::ControlApiClient::new(rc.clone())),
            listeners_api: Box::new(crate::apis::ListenersApiClient::new(rc.clone())),
            routes_api: Box::new(crate::apis::RoutesApiClient::new(rc.clone())),
            settings_api: Box::new(crate::apis::SettingsApiClient::new(rc.clone())),
            status_api: Box::new(crate::apis::StatusApiClient::new(rc.clone())),
            tls_api: Box::new(crate::apis::TlsApiClient::new(rc.clone())),
            xff_api: Box::new(crate::apis::XffApiClient::new(rc.clone())),
        }
    }

    pub fn access_log_api(&self) -> &dyn crate::apis::AccessLogApi {
        self.access_log_api.as_ref()
    }

    pub fn applications_api(&self) -> &dyn crate::apis::ApplicationsApi {
        self.applications_api.as_ref()
    }

    pub fn apps_api(&self) -> &dyn crate::apis::AppsApi {
        self.apps_api.as_ref()
    }

    pub fn certificates_api(&self) -> &dyn crate::apis::CertificatesApi {
        self.certificates_api.as_ref()
    }

    pub fn config_api(&self) -> &dyn crate::apis::ConfigApi {
        self.config_api.as_ref()
    }

    pub fn control_api(&self) -> &dyn crate::apis::ControlApi {
        self.control_api.as_ref()
    }

    pub fn listeners_api(&self) -> &dyn crate::apis::ListenersApi {
        self.listeners_api.as_ref()
    }

    pub fn routes_api(&self) -> &dyn crate::apis::RoutesApi {
        self.routes_api.as_ref()
    }

    pub fn settings_api(&self) -> &dyn crate::apis::SettingsApi {
        self.settings_api.as_ref()
    }

    pub fn status_api(&self) -> &dyn crate::apis::StatusApi {
        self.status_api.as_ref()
    }

    pub fn tls_api(&self) -> &dyn crate::apis::TlsApi {
        self.tls_api.as_ref()
    }

    pub fn xff_api(&self) -> &dyn crate::apis::XffApi {
        self.xff_api.as_ref()
    }
}
