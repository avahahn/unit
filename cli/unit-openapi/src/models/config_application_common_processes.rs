/*
 * NGINX Unit 1.31.1
 *
 * NGINX Unit is a lightweight and versatile application runtime that provides the essential components for your web application as a single open-source server: running application code, serving static assets, handling TLS and request routing.   **Important**: Ufnit's API is designed to expose any part of its configuration as an addressable endpoint.  Suppose a JSON object is stored at `/config/listeners/`:   ```json { \"*:8080\": { \"pass\": \"applications/wp_emea_dev\" } } ```  Here, `/config/listeners/_*:8080` and `/config/listeners/_*:8080/pass` are also endpoints.  Generally, object options are addressable by their names, array items—by their indexes (`/array/0/`).    **Note**: By default, Unit is configured through a UNIX domain socket. To use this specification with OpenAPI tools interactively, [start](https://unit.nginx.org/howto/source/#source-startup) Unit with a TCP port as the control socket.
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: unit-owner@nginx.org
 * Generated by: https://openapi-generator.tech
 */

/// ConfigApplicationCommonProcesses : Governs the behavior of app processes.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigApplicationCommonProcesses {
    /// Number of seconds Unit waits for before terminating an idle process that exceeds `spare`.
    #[serde(rename = "idle_timeout", skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// Maximum number of application processes that Unit maintains (busy and idle).
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Minimum number of idle processes that Unit tries to maintain for an app.
    #[serde(rename = "idle", skip_serializing_if = "Option::is_none")]
    pub idle: Option<i32>,
}

impl ConfigApplicationCommonProcesses {
    /// Governs the behavior of app processes.
    pub fn new() -> ConfigApplicationCommonProcesses {
        ConfigApplicationCommonProcesses {
            idle_timeout: None,
            max: None,
            idle: None,
        }
    }
}
