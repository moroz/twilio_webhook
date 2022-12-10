use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub fn hmac_sha1_base64<'a>(key: impl Into<&'a [u8]>, data: impl Into<&'a [u8]>) -> String {
    let mut mac = HmacSha1::new_from_slice(key.into()).unwrap();
    mac.update(data.into());
    let bytes = mac.finalize().into_bytes().to_vec();
    base64::encode(bytes)
}

#[test]
fn test_hmac_sha_hex() {
    let body = "bodystring";
    let key = "secret_key";
    let expected = "lwSWI7Dl0gv2vrUxPYBgDj1qvlY=";

    let actual = hmac_sha1_base64(key.as_bytes(), body.as_bytes());
    assert_eq!(actual, expected);
}
