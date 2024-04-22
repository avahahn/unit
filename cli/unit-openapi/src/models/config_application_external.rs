/*
 * NGINX Unit 1.31.1
 *
 * NGINX Unit is a lightweight and versatile application runtime that provides the essential components for your web application as a single open-source server: running application code, serving static assets, handling TLS and request routing.   **Important**: Ufnit's API is designed to expose any part of its configuration as an addressable endpoint.  Suppose a JSON object is stored at `/config/listeners/`:   ```json { \"*:8080\": { \"pass\": \"applications/wp_emea_dev\" } } ```  Here, `/config/listeners/_*:8080` and `/config/listeners/_*:8080/pass` are also endpoints.  Generally, object options are addressable by their names, array items—by their indexes (`/array/0/`).    **Note**: By default, Unit is configured through a UNIX domain socket. To use this specification with OpenAPI tools interactively, [start](https://unit.nginx.org/howto/source/#source-startup) Unit with a TCP port as the control socket.
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: unit-owner@nginx.org
 * Generated by: https://openapi-generator.tech
 */

/// ConfigApplicationExternal : Go or Node.js application on Unit.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigApplicationExternal {
    /// Application type and language version.
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Environment variables to be passed to the app.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// Group name that runs the app process.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Box<crate::models::ConfigApplicationCommonIsolation>>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Box<crate::models::ConfigApplicationCommonLimits>>,
    #[serde(rename = "processes", skip_serializing_if = "Option::is_none")]
    pub processes: Option<Box<crate::models::ConfigApplicationCommonProcesses>>,
    /// Username that runs the app process.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Filename where Unit redirects the app's stderr stream.
    #[serde(rename = "stderr", skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    /// Filename where Unit redirects the app's stdout stream.
    #[serde(rename = "stdout", skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    /// The app’s working directory.
    #[serde(rename = "working_directory", skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Pathname of the app, absolute or relative to `working_directory`.
    #[serde(rename = "executable")]
    pub executable: String,
    /// An array of strings.
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
}

impl ConfigApplicationExternal {
    /// Go or Node.js application on Unit.
    pub fn new(r#type: RHashType, executable: String) -> ConfigApplicationExternal {
        ConfigApplicationExternal {
            r#type,
            environment: None,
            group: None,
            isolation: None,
            limits: None,
            processes: None,
            user: None,
            stderr: None,
            stdout: None,
            working_directory: None,
            executable,
            arguments: None,
        }
    }
}

/// Application type and language version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "perl")]
    Perl,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "ruby")]
    Ruby,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::External
    }
}
