use crate::wait;
use crate::UnitctlError;
use std::io::Error as IoError;
use std::net::IpAddr;
use std::str::FromStr;

use std::borrow::ToOwned;
use std::convert::Infallible;

use crate::unitctl::UnitCtl;
use hyper::client::ResponseFuture;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Result as HyperResult, Server, StatusCode, Uri as HyperUri};
use hyper_tls::HttpsConnector;
use hyperlocal::{UnixClientExt, Uri as HyperLocalUri};
use std::net::SocketAddr;
use std::path::Path;
use unit_client_rs::control_socket_address::ControlSocket;

/// The HTML index page we serve from the UI server
const HTML_INDEX: &str = include_str!("../../www/index.html");
/// The minified Rapidoc JS library we serve from the UI server that reads the OpenAPI spec
/// and renders the UI
const RAPIDOC_MIN_JS: &str = include_str!("../../www/rapidoc-min.js");
/// The OpenAPI spec we serve from the UI server that Rapidoc reads to render the UI.
/// It is hardcoded here so that we do not need to make external network calls to fetch it.
const OPENAPI_SPEC: &str = include_str!("../../../../docs/unit-openapi.yaml");

pub(crate) fn cmd(cli: &UnitCtl, bind_address: &str, port: u16, debug: bool) -> Result<(), UnitctlError> {
    let bind_ip_address = IpAddr::from_str(bind_address).map_err(|e| UnitctlError::IoError {
        source: IoError::new(std::io::ErrorKind::InvalidInput, e),
    })?;
    let control_socket = wait::wait_for_socket(cli)?;
    start_ui(control_socket, bind_ip_address, port, debug)
}

/// Start the UI server
pub fn start_ui(
    control_socket: ControlSocket,
    bind_address: IpAddr,
    port: u16,
    debug: bool,
) -> Result<(), UnitctlError> {
    if debug {
        if control_socket.is_local_socket() {
            eprintln!("Using local socket: {}", control_socket);
        } else {
            eprintln!("Using tcp socket: {}", control_socket);
        }
    }

    let normalized_control_socket_address = control_socket.to_string();

    let ui_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create a current_thread runtime");

    ui_runtime
        .block_on(async move { run(normalized_control_socket_address, bind_address, port, debug).await })
        .map_err(|e| UnitctlError::UiServerError {
            message: format!("Error running UI: {}", e),
        })
}

#[derive(Debug, Clone)]
struct RequestForwarder {
    control_socket: String,
    rapidoc_request_path: String,
    debug: bool,
}

impl RequestForwarder {
    fn new(control_socket: String, debug: bool) -> Self {
        RequestForwarder {
            control_socket,
            rapidoc_request_path: format!("/rapidoc@{}/dist/rapidoc-min.js", Self::rapidoc_version()),
            debug,
        }
    }

    /// Get the version of the Rapidoc library we are serving from its minified JS file.
    fn rapidoc_version() -> &'static str {
        match RAPIDOC_MIN_JS.lines().next() {
            Some(first_line) => {
                let version = first_line
                    .strip_prefix("/*! RapiDoc ")
                    .map(|remaining| remaining.split_once(' ').unwrap().1);
                match version {
                    Some(version) => version,
                    None => panic!("Failed to parse RapiDoc version from rapidoc-min.js"),
                }
            }
            None => panic!("Failed to read first line of rapidoc-min.js"),
        }
    }

    /// Execute a client request against the UNIT control socket
    fn execute_client_request(&self, request: Request<Body>) -> ResponseFuture {
        let is_unix_local_socket = self.control_socket.starts_with("unix:");
        if is_unix_local_socket {
            Client::unix().request(request)
        } else {
            Client::builder().build(HttpsConnector::new()).request(request)
        }
    }

    /// Handle an incoming request to UNIT by forwarding it to the control socket
    async fn handle_request(&self, request: Request<Body>) -> HyperResult<Response<Body>> {
        // Build our request URI differently depending on if we are using a local unix socket
        let uri: HyperUri = match self.control_socket.strip_prefix("unix:") {
            Some(socket_path) => HyperLocalUri::new(Path::new(socket_path), request.uri().path()).into(),
            None => HyperUri::from_str(format!("{}{}", self.control_socket, request.uri().path()).as_str())
                .expect("Failed to build URI"),
        };
        let client_request = Request::builder()
            .method(request.method())
            .uri(uri)
            .body(request.into_body())
            .expect("Failed to build request");

        let response = self.execute_client_request(client_request).await;
        match response {
            Ok(mut response) => {
                response
                    .headers_mut()
                    .append("Cache-Control", "no-cache, no-store, must-revalidate".parse().unwrap());
                response.headers_mut().append("Pragma", "no-cache".parse().unwrap());
                response.headers_mut().append("Expires", "0".parse().unwrap());
                Ok(response)
            }
            Err(e) => {
                eprintln!("Error forwarding request: {:?}", e);
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap())
            }
        }
    }
}

/// Handle an incoming request to the UI server by routing it to the appropriate handler.
/// Generally all requests made outside of the /control-ui/ directory will be forwarded
/// directly to the control socket.
async fn request_router(req: Request<Body>, forwarder: RequestForwarder) -> HyperResult<Response<Body>> {
    if forwarder.debug {
        eprintln!("Request: {:?}", req.uri());
        eprintln!(" {:?}", req.headers());
    }

    // Remap the request path to serve the rapidoc-min.js file if requested with a hardcoded
    // version number. We do this so that we can easily change the version number of the
    // Rapidoc library.
    let path = if req.uri().path().eq(forwarder.rapidoc_request_path.as_str()) {
        eprintln!("Remapping request {} to /control-ui/rapidoc-min.js", req.uri().path());
        "/control-ui/rapidoc-min.js"
    } else {
        req.uri().path()
    };
    let response = match path {
        "/control-ui/" => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(HTML_INDEX.into())
            .unwrap()),
        "/control-ui/rapidoc-min.js" => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/javascript")
            .body(RAPIDOC_MIN_JS.into())
            .unwrap()),
        "/control-ui/unit-openapi.yaml" => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/x-yaml")
            .body(OPENAPI_SPEC.into())
            .unwrap()),
        "/favicon.ico" => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
        _ => forwarder.handle_request(req).await,
    };

    if forwarder.debug {
        if let Ok(response) = &response {
            eprintln!("Response status: {:?}", response.status());
            eprintln!("Response headers:\n {:?}", response.headers());
        } else {
            eprintln!("Response error: {:?}", response);
        }
    }

    response
}

/// Start the UI server using Tokio
async fn run(control_socket: String, bind_address: IpAddr, port: u16, debug: bool) -> HyperResult<()> {
    // Set the address to run our server on
    let addr = SocketAddr::new(bind_address, port);
    eprintln!("Starting UI server on http://{}/control-ui/", addr);
    eprintln!("Press Ctrl-C to stop the server");

    let make_service = make_service_fn(move |_| {
        let request_handler = RequestForwarder::new(control_socket.to_owned(), debug.to_owned());

        async {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            Ok::<_, Infallible>(service_fn(move |req| request_router(req, request_handler.to_owned())))
        }
    });

    Server::bind(&addr).serve(make_service).await
}
