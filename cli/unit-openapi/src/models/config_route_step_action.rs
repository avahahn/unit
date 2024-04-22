/*
 * NGINX Unit 1.31.1
 *
 * NGINX Unit is a lightweight and versatile application runtime that provides the essential components for your web application as a single open-source server: running application code, serving static assets, handling TLS and request routing.   **Important**: Ufnit's API is designed to expose any part of its configuration as an addressable endpoint.  Suppose a JSON object is stored at `/config/listeners/`:   ```json { \"*:8080\": { \"pass\": \"applications/wp_emea_dev\" } } ```  Here, `/config/listeners/_*:8080` and `/config/listeners/_*:8080/pass` are also endpoints.  Generally, object options are addressable by their names, array items—by their indexes (`/array/0/`).    **Note**: By default, Unit is configured through a UNIX domain socket. To use this specification with OpenAPI tools interactively, [start](https://unit.nginx.org/howto/source/#source-startup) Unit with a TCP port as the control socket.
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: unit-owner@nginx.org
 * Generated by: https://openapi-generator.tech
 */

/// ConfigRouteStepAction : An object whose options define a step's action.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigRouteStepAction {
    /// Destination to which the action passes incoming requests.
    #[serde(rename = "pass")]
    pub pass: String,
    /// Updates the URI of the incoming request before the action is applied.
    #[serde(rename = "rewrite", skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<String>,
    /// Updates the header fields of Unit’s response before the action is taken.
    #[serde(rename = "response_headers", skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<::std::collections::HashMap<String, String>>,
    /// Socket address of an HTTP server to where the request is proxied.
    #[serde(rename = "proxy")]
    pub proxy: String,
    /// Defines the HTTP response status code to be returned.
    #[serde(rename = "return")]
    pub r#return: i32,
    /// URI; used if the return value implies redirection.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "share")]
    pub share: Box<crate::models::StringOrStringArray>,
    /// Filename; tried if share is a directory.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Box<crate::models::ConfigRouteStepAction>>,
    /// An array of strings.
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// Directory pathname that restricts the shareable paths.
    #[serde(rename = "chroot", skip_serializing_if = "Option::is_none")]
    pub chroot: Option<String>,
    /// Turns on and off symbolic link resolution.
    #[serde(rename = "follow_symlinks", skip_serializing_if = "Option::is_none")]
    pub follow_symlinks: Option<bool>,
    /// Turns on and off mount point resolution.
    #[serde(rename = "traverse_mounts", skip_serializing_if = "Option::is_none")]
    pub traverse_mounts: Option<bool>,
}

impl ConfigRouteStepAction {
    /// An object whose options define a step's action.
    pub fn new(
        pass: String,
        proxy: String,
        r#return: i32,
        share: crate::models::StringOrStringArray,
    ) -> ConfigRouteStepAction {
        ConfigRouteStepAction {
            pass,
            rewrite: None,
            response_headers: None,
            proxy,
            r#return,
            location: None,
            share: Box::new(share),
            index: None,
            fallback: None,
            types: None,
            chroot: None,
            follow_symlinks: None,
            traverse_mounts: None,
        }
    }
}
