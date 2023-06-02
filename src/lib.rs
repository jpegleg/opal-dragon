#[cfg(test)]
mod tests {

    #[test]
    fn uuidtest() {
      use uuid::Uuid;
      assert_eq!(Uuid::new_v4().to_string().is_empty(), false);
    }

    #[test]
    fn datetest() {
      use chrono::prelude::*;
      assert_eq!(Utc::now().to_string().is_empty(), false);
      let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28).unwrap().and_hms_nano_opt(12, 0, 9, 1).unwrap().and_local_timezone(Utc).unwrap();
      assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
    }

    #[test]
    fn base64test() {
      use base64::{Engine as _};
      extern crate base64;
      let orig = b"Wowza mihno!";
      let encoded: String = base64::engine::general_purpose::STANDARD_NO_PAD.encode(orig);
      assert_eq!("V293emEgbWlobm8h", encoded);
      assert_eq!(orig.as_slice(), &base64::engine::general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap());
    }

    #[test]
    fn genkeytest() {
      use rand::rngs::OsRng;
      use ed25519_dalek::{Keypair, PUBLIC_KEY_LENGTH};
      extern crate rand;
      extern crate ed25519_dalek;
      let mut csprng = OsRng{};
      let keypair: Keypair = Keypair::generate(&mut csprng);
      let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes(); 
      let mut csprng2 = OsRng{};
      let keypair2: Keypair = Keypair::generate(&mut csprng2);
      let public_key_bytes2: [u8; PUBLIC_KEY_LENGTH] = keypair2.public.to_bytes(); 
      assert_ne!(public_key_bytes, public_key_bytes2);
   }

    #[test]
    fn dalektest() {
      extern crate ed25519_dalek;
      extern crate rand;
      use rand::rngs::OsRng;
      use ed25519_dalek::{Keypair, Signature, Signer};
      let mut csprng = OsRng{};
      let keypair: Keypair = Keypair::generate(&mut csprng);
      let message: &[u8] = b"This is a test.";
      let signature: Signature = keypair.sign(message);
      assert!(keypair.verify(message, &signature).is_ok());
  }

}
