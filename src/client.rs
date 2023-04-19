use std::io::stdin;
use opal::{opal_client::OpalClient, OpalRequest};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

pub mod opal {
  tonic::include_proto!("opal");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_root_ca_cert = std::fs::read_to_string("opal_server_ca.pem")?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = std::fs::read_to_string("opal_client_access.pem")?;
    let client_key = std::fs::read_to_string("opal_client_access.key")?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let channel = Channel::from_static("https://127.0.0.1:3042")
        .tls_config(tls)?
        .connect()
        .await?;

   
    let mut client = OpalClient::new(channel);

    println!("\nStarted opal_dragon client session.\n");
    let mut u = String::new();
    let mut p = String::new();
    let mut w = String::new();

    let mut fire: String = String::new();
    println!("Enter a message: ");
    stdin().read_line(&mut u).unwrap();
    let u = u.trim();
    println!("To verify, provide a public key, base64 encoded, otherwise leave empty and hit return (newline): ");
    stdin().read_line(&mut p).unwrap();
    let p = p.trim();
    println!("To verify, provide a signature, hex encoded, otherwise leave empty and hit return (newline): ");
    stdin().read_line(&mut w).unwrap();
    let w = w.trim();
    println!("Select an option, (s)ign or (v)erify: ");
    stdin().read_line(&mut fire).unwrap();
    let v = match fire.trim().to_lowercase().chars().next().unwrap()
        {
          's' => 0,
          'v' => 1,
          _ => 2,
        };
    let request = tonic::Request::new(OpalRequest {
        inpoot: String::from(u),
        publick: String::from(p),
        signat: String::from(w),
        fire: v,
    });
    let response = client.fire(request).await?;
    println!("opal_dragon: '{}'\n", response.into_inner().confirmation);
    Ok(())
}
