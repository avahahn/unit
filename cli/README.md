# NGINX UNIT Rust SDK and CLI

This project provides a Rust SDK interface to the [NGINX UNIT](https://unit.nginx.org/)
[control API](https://unit.nginx.org/howto/source/#source-startup) and a CLI that exposes
the functionality provided by the SDK.

## Vision

There are two main goals for this project:

1. Provide a Rust SDK for the NGINX UNIT control API that has been pregenerated from the
   OpenAPI specification. This will allow Rust developers to easily integrate NGINX UNIT
   into their applications without having to generate their own SDK from the OpenAPI
   [specification file](https://unit.nginx.org/controlapi/#openapi-specification).

2. Provide a CLI that exposes the functionality of the SDK. This will allow users to
   interact with NGINX UNIT from the command line without having to write their own
   scripts or programs. The CLI will also provide a reference implementation of the SDK.
   Moreover, the CLI will integrate in data format conversions and validations that are
   not supported by NGINX UNIT.

## Features (Current)

### Parses and validates configuration formats that UNIT does not support like JSON5 and YAML and converts them before sending to UNIT
### Syntactic highlighting of JSON output
### Interpretation of UNIT errors with (arguably more) useful error messages

### Lists all running UNIT processes and provides details about each process.
```
$ unitctl instances
No socket path provided - attempting to detect from running instance
unitd instance [pid: 79489, version: 1.32.0]:
  Executable: /opt/unit/sbin/unitd
  API control unix socket: unix:/opt/unit/control.unit.sock
  Child processes ids: 79489, 79489
  Runtime flags: --no-daemon
  Configure options: --prefix=/opt/unit --user=elijah --group=elijah --openssl --debug
```

### Lists active listeners from running UNIT processes
```
unitctl listeners
No socket path provided - attempting to detect from running instance
{
  "127.0.0.1:8080": {
    "pass": "routes"
  }
}
```

### Get the current status of NGINX UNIT processes
```
$ unitctl status -t yaml
No socket path provided - attempting to detect from running instance
connections:
  accepted: 0
  active: 0
  idle: 0
  closed: 0
requests:
  total: 0
applications: {}
```

### Send arbitrary configuration payloads to UNIT
```
$ echo '{
    "listeners": {
        "127.0.0.1:8080": {
            "pass": "routes"
        }
    },

    "routes": [
        {
            "action": {
                "share": "/www/data$uri"
            }
        }
    ]
}' | unitctl execute --http-method PUT --path /config -f -
{
  "success": "Reconfiguration done."
}
```

### Edit current configuration in your favorite editor
```
$ unitctl edit
[[EDITOR LOADS SHOWING CURRENT CONFIGURATION - USER EDITS AND SAVES]]

{
  "success": "Reconfiguration done."
}       
```

### Display interactive OpenAPI control panel
```
$ unitctl ui
Starting UI server on http://127.0.0.1:3000/control-ui/
Press Ctrl-C to stop the server
```

### Import configuration, certificates, and NJS modules from directory
```
$ unitctl import /opt/unit/config
Imported /opt/unit/config/certificates/consolidated_snake.pem -> /certificates/consolidated_snake.pem
Imported /opt/unit/config/hello.js -> /js_modules/hello.js
Imported /opt/unit/config/put.json -> /config
Imported 3 files
```
### Wait for socket to become available
```
$ unitctl --wait-timeout-seconds=3 --wait-max-tries=4 import /opt/unit/config`
Waiting for 3s control socket to be available try 2/4...
Waiting for 3s control socket to be available try 3/4...
Waiting for 3s control socket to be available try 4/4...
Timeout waiting for unit to start has been exceeded
```

## Features (Proposed)
* Certificate validation (expiration date and validity checks) before upload
* Make adding and maintaining mime types easier
* Add features present in unit-cli

## Contribution

We welcome pull requests and issues!

Please refer to the [Contributing Guidelines](../CONTRIBUTING.md) when doing a PR.

## License

All code in this repository is licensed under the
[Apache License v2 license](../LICENSE.txt).
