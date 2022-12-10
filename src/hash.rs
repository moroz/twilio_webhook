use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub fn hmac_sha1_base64<'a>(key: impl Into<&'a [u8]>, data: impl Into<&'a [u8]>) -> String {
    let mut mac = HmacSha1::new_from_slice(key.into()).unwrap();
    mac.update(data.into());
    let bytes = mac.finalize().into_bytes().to_vec();
    base64::encode(bytes)
}

pub fn validate_body(body: String, signature: String) -> bool {
    unimplemented!()
}

pub fn get_sha_hash_from_url(url: String) -> Option<String> {
    let parsed = url::Url::parse(url.as_str());
    if let Err(_) = parsed {
        return None;
    }
    let parsed = parsed.unwrap();
    let pairs = parsed.query_pairs();
    for (key, value) in pairs {
        if key == "bodySHA256" {
            return Some(value.to_owned().to_string());
        }
    }
    return None;
}

pub fn validate_request_with_body(
    auth_token: String,
    signature: String,
    url: String,
    body: String,
) -> bool {
    let sha_hash = get_sha_hash_from_url(url);
    if sha_hash.is_none() {
        return false;
    }

    validate_body(body, sha_hash.unwrap());
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash_from_url() {
        let body_signature = "0a1ff7634d9ab3b95db5c9a2dfe9416e41502b283a80c7cf19632632f96e6620";
        let request_url = "https://mycompany.com/myapp.php?foo=1&bar=2";
        let request_url_with_hash = format!("{}&bodySHA256={}", request_url, body_signature);
        let actual = get_sha_hash_from_url(request_url_with_hash).unwrap();
        assert_eq!(actual, body_signature.to_string());
    }

    #[test]
    fn test_hmac_sha_hex() {
        let body = "bodystring";
        let key = "secret_key";
        let expected = "lwSWI7Dl0gv2vrUxPYBgDj1qvlY=";

        let actual = hmac_sha1_base64(key.as_bytes(), body.as_bytes());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_validate_request_with_body() {
        let body = r#"{"property": "value", "boolean": true}"#.to_string();
        let body_signature =
            "0a1ff7634d9ab3b95db5c9a2dfe9416e41502b283a80c7cf19632632f96e6620".to_string();
        let request_url = "https://mycompany.com/myapp.php?foo=1&bar=2".to_string();
        let request_url_with_hash = format!("{}&bodySHA256={}", request_url, body_signature);
        let request_url_with_hash_signature = "a9nBmqA0ju/hNViExpshrM61xv4=".to_string();

        let actual = validate_request_with_body(
            "12345".to_string(),
            request_url_with_hash_signature,
            request_url_with_hash,
            body,
        );

        assert!(actual);
    }
}
