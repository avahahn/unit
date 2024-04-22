/*
 * NGINX Unit 1.31.1
 *
 * NGINX Unit is a lightweight and versatile application runtime that provides the essential components for your web application as a single open-source server: running application code, serving static assets, handling TLS and request routing.   **Important**: Ufnit's API is designed to expose any part of its configuration as an addressable endpoint.  Suppose a JSON object is stored at `/config/listeners/`:   ```json { \"*:8080\": { \"pass\": \"applications/wp_emea_dev\" } } ```  Here, `/config/listeners/_*:8080` and `/config/listeners/_*:8080/pass` are also endpoints.  Generally, object options are addressable by their names, array items—by their indexes (`/array/0/`).    **Note**: By default, Unit is configured through a UNIX domain socket. To use this specification with OpenAPI tools interactively, [start](https://unit.nginx.org/howto/source/#source-startup) Unit with a TCP port as the control socket.
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: unit-owner@nginx.org
 * Generated by: https://openapi-generator.tech
 */

/// ConfigListenerForwarded : Configures client IP address and protocol replacement.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigListenerForwarded {
    /// Defines the HTTP header fields to expect in the request; uses the `X-Forwarded-For` format.
    #[serde(rename = "client_ip", skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(rename = "source")]
    pub source: Box<crate::models::ConfigListenerForwardedSource>,
    /// Controls how the `client_ip` fields are traversed.
    #[serde(rename = "recursive", skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// Defines the relevant HTTP header field to expect in the request; uses the `X-Forwarded-Proto` format.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}

impl ConfigListenerForwarded {
    /// Configures client IP address and protocol replacement.
    pub fn new(source: crate::models::ConfigListenerForwardedSource) -> ConfigListenerForwarded {
        ConfigListenerForwarded {
            client_ip: None,
            source: Box::new(source),
            recursive: None,
            protocol: None,
        }
    }
}

/// Defines the relevant HTTP header field to expect in the request; uses the `X-Forwarded-Proto` format.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "on")]
    On,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Http
    }
}
