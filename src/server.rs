use tonic::{transport::Server, Request, Response, Status};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};
use rand::rngs::OsRng;
use uuid::Uuid;
use chrono::prelude::*;
use base64::{Engine as _};
use opal::{OpalRequest, OpalResponse, opal_server::{Opal, OpalServer}};
use ed25519_dalek::{Signature, Signer, Keypair, Verifier, PublicKey, PUBLIC_KEY_LENGTH};

extern crate base64;
extern crate rand;
extern crate ed25519_dalek;

pub mod opal {
  tonic::include_proto!("opal");
}

#[derive(Debug, Default)]
pub struct OpalService {}

#[tonic::async_trait]
impl Opal for OpalService {
  async fn fire(&self, request: Request<OpalRequest>) -> Result<Response<OpalResponse>, Status> {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let pub64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(public_key_bytes);
    let txid = Uuid::new_v4().to_string();
    let readi: DateTime<Utc> = Utc::now();
    println!("{} {} opal_dragon - Public key used: {:?}", &readi, &txid, &pub64);

    let r = request.into_inner();
    match r.fire {
      0 => Ok(Response::new(opal::OpalResponse { confirmation: { 
        let message: &[u8] = r.inpoot.as_bytes();
        let signature: Signature = keypair.sign(message);
        format!("{} {} {:?} {:?}", txid, r.inpoot, signature, pub64)
      }})),
      1 => Ok(Response::new(opal::OpalResponse { confirmation: { 
        let message: &[u8] = r.inpoot.as_bytes();
        let readn: DateTime<Utc> = Utc::now();
        println!("{} {} opal_dragon - DEBUG message: {:?}", &readn, &txid, &message);
        let upubkey = base64::engine::general_purpose::STANDARD_NO_PAD.decode(r.publick).unwrap();
        println!("{} {} opal_dragon - DEBUG public key: {:?}", &readn, &txid, &upubkey);
        let pubkey = PublicKey::from_bytes(&upubkey).unwrap();
        println!("{} {} opal_dragon - DEBUG public key bytes: {:?}", &readn, &txid, &pubkey);
        let sigproc = r.signat.as_bytes();
        println!("{} {} opal_dragon - DEBUG signature bytes: {:?}", &readn, &txid, &sigproc);
        let dehexsign = hex::decode(sigproc).unwrap();
        let esigna: Signature = Signature::from_bytes(&dehexsign).unwrap();
        println!("{} {} opal_dragon - DEBUG signature: {:?}", &readn, &txid, &esigna);
        let veri = pubkey.verify(message, &esigna).is_ok();
        println!("{} {} opal_dragon - DEBUG verify: {:?}", &readn, &txid, &veri);
        format!("{} {}", txid, veri)
      }})), 
      _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid provided from client"))
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cert = std::fs::read_to_string("server.pem")?;
  let key = std::fs::read_to_string("server.key")?;
  let server_identity = Identity::from_pem(cert, key);
  let client_ca_cert = std::fs::read_to_string("client_ca.pem")?;
  let client_ca_cert = Certificate::from_pem(client_ca_cert);
  let hostport = "0.0.0.0:3042";
  let addr = hostport.parse().unwrap();
  let server = OpalService::default();

  let tls = ServerTlsConfig::new()
      .identity(server_identity)
      .client_ca_root(client_ca_cert);

  Server::builder()
      .tls_config(tls)?
      .add_service(OpalServer::new(server))
      .serve(addr)
      .await?;

  Ok(())
     
}
