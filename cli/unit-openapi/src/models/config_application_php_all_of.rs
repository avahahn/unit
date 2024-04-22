/*
 * NGINX Unit 1.31.1
 *
 * NGINX Unit is a lightweight and versatile application runtime that provides the essential components for your web application as a single open-source server: running application code, serving static assets, handling TLS and request routing.   **Important**: Ufnit's API is designed to expose any part of its configuration as an addressable endpoint.  Suppose a JSON object is stored at `/config/listeners/`:   ```json { \"*:8080\": { \"pass\": \"applications/wp_emea_dev\" } } ```  Here, `/config/listeners/_*:8080` and `/config/listeners/_*:8080/pass` are also endpoints.  Generally, object options are addressable by their names, array items—by their indexes (`/array/0/`).    **Note**: By default, Unit is configured through a UNIX domain socket. To use this specification with OpenAPI tools interactively, [start](https://unit.nginx.org/howto/source/#source-startup) Unit with a TCP port as the control socket.
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: unit-owner@nginx.org
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigApplicationPhpAllOf {
    /// Base directory of the app’s file structure.
    #[serde(rename = "root")]
    pub root: String,
    /// Filename added to URI paths that point to directories if no `script` is set.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::ConfigApplicationPhpAllOfOptions>>,
    /// Filename of a `root`-based PHP script that serves all requests to the app.
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    /// Application sections with custom `root`, `script`, and `index` values.
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<::std::collections::HashMap<String, crate::models::ConfigApplicationPhpAllOfTargets>>,
}

impl ConfigApplicationPhpAllOf {
    pub fn new(root: String) -> ConfigApplicationPhpAllOf {
        ConfigApplicationPhpAllOf {
            root,
            index: None,
            options: None,
            script: None,
            targets: None,
        }
    }
}
