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
      extern crate base64;
      let orig = b"Wowza mihno!";
      let encoded: String = base64::encode(orig);
      assert_eq!("V293emEgbWlobm8h", encoded);
      assert_eq!(orig.as_slice(), &base64::decode(encoded).unwrap());
    }

}
