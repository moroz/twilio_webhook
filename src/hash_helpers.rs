use hmac::{Hmac, Mac};
use sha1::Sha1;

pub type HmacSha1 = Hmac<Sha1>;

pub fn hmac_sha1_base64<'a>(key: &'a str, data: impl Into<&'a [u8]>) -> String {
    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data.into());
    let bytes = mac.finalize().into_bytes().to_vec();
    base64::encode(bytes)
}
