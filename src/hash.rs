use std::{any::Any, collections::BTreeMap};

use crate::{add_port, remove_port};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha256::digest as sha256digest;

type HmacSha1 = Hmac<Sha1>;

pub fn hmac_sha1_base64<'a>(key: impl Into<&'a [u8]>, data: impl Into<&'a [u8]>) -> String {
    let mut mac = HmacSha1::new_from_slice(key.into()).unwrap();
    mac.update(data.into());
    let bytes = mac.finalize().into_bytes().to_vec();
    base64::encode(bytes)
}

pub fn validate_json_body(body: String, expected_signature: String) -> bool {
    let signature = sha256digest(body);
    signature == expected_signature
}

pub fn get_sha_hash_from_url(url: String) -> Result<Option<String>, url::ParseError> {
    let parsed = url::Url::parse(url.as_str());
    if let Err(err) = parsed {
        return Err(err);
    }
    let parsed = parsed.unwrap();
    let pairs = parsed.query_pairs();
    for (key, value) in pairs {
        if key == "bodySHA256" {
            return Ok(Some(value.to_owned().to_string()));
        }
    }
    return Ok(None);
}

pub fn get_expected_twilio_signature<'a>(
    auth_token: &'a str,
    url: String,
    params: &Vec<String>,
) -> String {
    let mut builder: Vec<u8> = Vec::with_capacity(1024);
    builder.extend_from_slice(url.as_bytes());
    for param in params {
        builder.extend_from_slice(param.as_bytes());
    }
    return hmac_sha1_base64(auth_token.as_bytes(), builder.as_slice());
}

pub fn parse_and_sort_urlencoded_body<'a>(body: &'a str) -> Vec<String> {
    let parsed = form_urlencoded::parse(body.as_bytes());
    let mut as_vec: Vec<_> = parsed.collect();
    as_vec.sort_by_key(move |(k, _v)| k.clone());
    as_vec.iter().map(|(k, v)| format!("{}{}", k, v)).collect()
}

pub fn validate_body(
    auth_token: String,
    url: String,
    body: String,
    expected_signature: String,
) -> bool {
    unimplemented!()
}

pub fn validate_urlencoded<'a>(
    auth_token: &'a str,
    signature: &'a str,
    url: &'a str,
    body: &'a str,
) -> bool {
    let params = parse_and_sort_urlencoded_body(body);
    return validate_url(auth_token, signature, url, params);
}

pub fn validate_url<'a>(
    auth_token: &'a str,
    signature: &'a str,
    url: &'a str,
    params: Vec<String>,
) -> bool {
    let parsed_url = url::Url::parse(url);
    if let Err(_) = parsed_url {
        return false;
    }
    let parsed_url = parsed_url.unwrap();
    let signature_with_port =
        get_expected_twilio_signature(auth_token, add_port(&parsed_url), &params);
    let signature_without_port =
        get_expected_twilio_signature(auth_token, remove_port(&parsed_url), &params);
    return signature_with_port == signature || signature_without_port == signature;
}

pub fn validate_request_with_body<'a>(
    auth_token: &'a str,
    signature: &'a str,
    url: &'a str,
    body: &'a str,
) -> bool {
    // If the request has a bodySHA256 param, it is in JSON
    match get_sha_hash_from_url(url.to_string()) {
        Err(_) => {
            // Invalid URL
            return false;
        }
        Ok(None) => {
            // URL encoded form
            return validate_urlencoded(auth_token, signature, url, body);
        }
        Ok(Some(sha_hash)) => {
            return validate_url(auth_token, signature, url, vec![])
                && validate_json_body(body.to_string(), sha_hash);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash_from_url() {
        let body_signature =
            "0a1ff7634d9ab3b95db5c9a2dfe9416e41502b283a80c7cf19632632f96e6620".to_string();
        let request_url = "https://mycompany.com/myapp.php?foo=1&bar=2";
        let request_url_with_hash = format!("{}&bodySHA256={}", request_url, body_signature);
        let actual = get_sha_hash_from_url(request_url_with_hash);
        assert_eq!(actual, Ok(Some(body_signature)));
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
        let body = r#"{"property": "value", "boolean": true}"#;
        let body_signature =
            "0a1ff7634d9ab3b95db5c9a2dfe9416e41502b283a80c7cf19632632f96e6620".to_string();
        let request_url = "https://mycompany.com/myapp.php?foo=1&bar=2".to_string();
        let request_url_with_hash = format!("{}&bodySHA256={}", request_url, body_signature);
        let request_url_with_hash_signature = "a9nBmqA0ju/hNViExpshrM61xv4=";

        let actual = validate_request_with_body(
            "12345",
            request_url_with_hash_signature,
            request_url_with_hash.as_str(),
            body,
        );

        assert!(actual);
    }

    #[test]
    fn test_validate_urlencoded_request() {
        let token = "c73504dac708a5cd9f57e80c747bb488";
        let signature = "cN6s/ajWzahiBNHjFpssnkbSQSM=";
        let url = "https://0447-85-232-252-1.eu.ngrok.io/twilio/conference_status?waiter_id=42";
        let body = r#"AccountSid=ACe497b94cea336b5d573d9667ffda50bf&AddOns=%7B+%22status%22%3A+%22successful%22%2C+%22message%22%3A+null%2C+%22code%22%3A+null%2C+%22results%22%3A+%7B+%7D+%7D&ApiVersion=2010-04-01&From=%2B15017122661&FromCity=SAN+FRANCISCO&FromCountry=US&FromState=CA&FromZip=94903&To=%2B15558675310&ToCity=SAN+FRANCISCO&ToCountry=US&ToState=CA&ToZip=94105&Body=Ahoy&MessageSid=SMaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa&NumMedia=0&NumSegments=1&ReferralNumMedia=0&SmsMessageSid=SMaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa&SmsSid=SMaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa&SmsStatus=received"#;

        let actual = validate_request_with_body(token, signature, url, body);

        assert!(actual);
    }
}
