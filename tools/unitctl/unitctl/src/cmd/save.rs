use crate::unitctl::UnitCtl;
use crate::wait;
use crate::UnitctlError;
use crate::requests::send_empty_body_deserialize_response;
use unit_client_rs::unit_client::UnitClient;
use tar::{Builder, Header};
use std::fs::File;
use std::io::stdout;


pub async fn cmd(
    cli: &UnitCtl,
    filename: &String
) -> Result<(), UnitctlError> {
    if !filename.ends_with(".tar") {
        eprintln!("Warning: writing uncompressed tarball to {}", filename);
    }

    let control_socket = wait::wait_for_socket(cli).await?;
    let client = UnitClient::new(control_socket);

    let config_res = serde_json::to_string_pretty(
        &send_empty_body_deserialize_response(&client, "GET", "/config").await?
    );
    if let Err(e) = config_res {
        return Err(UnitctlError::DeserializationError{message: e.to_string()})
    }

    let current_config = config_res
        .unwrap()
        .into_bytes();

    //let current_js_modules = send_empty_body_deserialize_response(&client, "GET", "/js_modules")
    //    .await?;

    let mut conf_header = Header::new_gnu();
    conf_header.set_size(current_config.len() as u64);
    conf_header.set_mode(0o644);
    conf_header.set_cksum();

    // builder has a different type depending on output
    if filename == "-" {
        let mut ar = Builder::new(stdout());
        ar.append_data(&mut conf_header, "config.json", current_config.as_slice()).unwrap();
    } else {
        let file = File::create(filename).unwrap();
        let mut ar = Builder::new(file);
        ar.append_data(&mut conf_header, "config.json", current_config.as_slice()).unwrap();
    }

    Ok(())
}
