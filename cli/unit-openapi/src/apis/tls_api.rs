/*
 * NGINX Unit 1.31.1
 *
 * NGINX Unit is a lightweight and versatile application runtime that provides the essential components for your web application as a single open-source server: running application code, serving static assets, handling TLS and request routing.   **Important**: Ufnit's API is designed to expose any part of its configuration as an addressable endpoint.  Suppose a JSON object is stored at `/config/listeners/`:   ```json { \"*:8080\": { \"pass\": \"applications/wp_emea_dev\" } } ```  Here, `/config/listeners/_*:8080` and `/config/listeners/_*:8080/pass` are also endpoints.  Generally, object options are addressable by their names, array items—by their indexes (`/array/0/`).    **Note**: By default, Unit is configured through a UNIX domain socket. To use this specification with OpenAPI tools interactively, [start](https://unit.nginx.org/howto/source/#source-startup) Unit with a TCP port as the control socket.
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: unit-owner@nginx.org
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::pin::Pin;
use std::rc::Rc;

use futures::Future;
use hyper;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct TlsApiClient<C: hyper::client::connect::Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> TlsApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TlsApiClient<C> {
        TlsApiClient { configuration }
    }
}

pub trait TlsApi {
    fn delete_listener_tls(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn delete_listener_tls_certificate(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn delete_listener_tls_certificates(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn delete_listener_tls_conf_commands(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn delete_listener_tls_session(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn delete_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn delete_listener_tls_session_tickets(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn get_listener_tls(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTls, Error>>>>;
    fn get_listener_tls_certificate(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn get_listener_tls_session(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTlsSession, Error>>>>;
    fn get_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn insert_listener_tls_certificate(
        &self,
        listener_name: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn insert_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn list_listener_tls_certificates(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTlsCertificate, Error>>>>;
    fn list_listener_tls_conf_commands(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn list_listener_tls_session_tickets(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTlsSessionTickets, Error>>>>;
    fn update_listener_tls(
        &self,
        listener_name: &str,
        config_listener_tls: crate::models::ConfigListenerTls,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn update_listener_tls_certificate(
        &self,
        listener_name: &str,
        array_index: i32,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn update_listener_tls_certificates(
        &self,
        listener_name: &str,
        string_or_string_array: Option<crate::models::StringOrStringArray>,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn update_listener_tls_conf_commands(
        &self,
        listener_name: &str,
        request_body: ::std::collections::HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn update_listener_tls_session(
        &self,
        listener_name: &str,
        config_listener_tls_session: crate::models::ConfigListenerTlsSession,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn update_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        array_index: i32,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
    fn update_listener_tls_session_tickets(
        &self,
        listener_name: &str,
        config_listener_tls_session_tickets: Option<crate::models::ConfigListenerTlsSessionTickets>,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>>;
}

impl<C: hyper::client::connect::Connect> TlsApi for TlsApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn delete_listener_tls(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_listener_tls_certificate(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls/certificate/{arrayIndex}".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_path_param("arrayIndex".to_string(), array_index.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_listener_tls_certificates(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls/certificate".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_listener_tls_conf_commands(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls/conf_commands".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_listener_tls_session(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls/session".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls/session/tickets/{arrayIndex}".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_path_param("arrayIndex".to_string(), array_index.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_listener_tls_session_tickets(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/config/listeners/{listenerName}/tls/session/tickets".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_listener_tls(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTls, Error>>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/config/listeners/{listenerName}/tls".to_string());
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_listener_tls_certificate(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/config/listeners/{listenerName}/tls/certificate/{arrayIndex}".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_path_param("arrayIndex".to_string(), array_index.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_listener_tls_session(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTlsSession, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/config/listeners/{listenerName}/tls/session".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        array_index: i32,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/config/listeners/{listenerName}/tls/session/tickets/{arrayIndex}".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_path_param("arrayIndex".to_string(), array_index.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn insert_listener_tls_certificate(
        &self,
        listener_name: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/config/listeners/{listenerName}/tls/certificate".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn insert_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/config/listeners/{listenerName}/tls/session/tickets".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn list_listener_tls_certificates(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTlsCertificate, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/config/listeners/{listenerName}/tls/certificate".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn list_listener_tls_conf_commands(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/config/listeners/{listenerName}/tls/conf_commands".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn list_listener_tls_session_tickets(
        &self,
        listener_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::models::ConfigListenerTlsSessionTickets, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/config/listeners/{listenerName}/tls/session/tickets".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls(
        &self,
        listener_name: &str,
        config_listener_tls: crate::models::ConfigListenerTls,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/config/listeners/{listenerName}/tls".to_string());
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(config_listener_tls);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls_certificate(
        &self,
        listener_name: &str,
        array_index: i32,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/config/listeners/{listenerName}/tls/certificate/{arrayIndex}".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_path_param("arrayIndex".to_string(), array_index.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls_certificates(
        &self,
        listener_name: &str,
        string_or_string_array: Option<crate::models::StringOrStringArray>,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/config/listeners/{listenerName}/tls/certificate".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(string_or_string_array);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls_conf_commands(
        &self,
        listener_name: &str,
        request_body: ::std::collections::HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/config/listeners/{listenerName}/tls/conf_commands".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(request_body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls_session(
        &self,
        listener_name: &str,
        config_listener_tls_session: crate::models::ConfigListenerTlsSession,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/config/listeners/{listenerName}/tls/session".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(config_listener_tls_session);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls_session_ticket(
        &self,
        listener_name: &str,
        array_index: i32,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/config/listeners/{listenerName}/tls/session/tickets/{arrayIndex}".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_path_param("arrayIndex".to_string(), array_index.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_listener_tls_session_tickets(
        &self,
        listener_name: &str,
        config_listener_tls_session_tickets: Option<crate::models::ConfigListenerTlsSessionTickets>,
    ) -> Pin<Box<dyn Future<Output = Result<::std::collections::HashMap<String, String>, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/config/listeners/{listenerName}/tls/session/tickets".to_string(),
        );
        req = req.with_path_param("listenerName".to_string(), listener_name.to_string());
        req = req.with_body_param(config_listener_tls_session_tickets);

        req.execute(self.configuration.borrow())
    }
}
