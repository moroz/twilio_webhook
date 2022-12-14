use url::Url;

pub fn build_url_with_standard_port(url: &Url) -> String {
    let parsed = url.clone();
    let normalized_host = match parsed.host_str() {
        Some(host) => format!("{}:{}", host, parsed.port_or_known_default().unwrap()),
        _ => "".to_string(),
    };
    let normalized_query = match parsed.query() {
        Some(query) => format!("?{query}"),
        _ => "".to_string(),
    };
    let normalized_fragment = match parsed.fragment() {
        Some(fragment) => format!("#{fragment}"),
        _ => "".to_string(),
    };
    return format!(
        "{}://{}{}{}{}",
        parsed.scheme(),
        normalized_host,
        parsed.path(),
        normalized_query,
        normalized_fragment
    );
}

pub fn add_port(url: &Url) -> String {
    if let Some(_) = url.port() {
        return url.to_string();
    }
    build_url_with_standard_port(url)
}

pub fn remove_port(url: &Url) -> String {
    let mut new_url = url.clone();
    new_url.set_port(None).unwrap();
    new_url.to_string()
}

pub fn get_sha_hash_from_url<'a>(url: &'a str) -> Result<Option<String>, url::ParseError> {
    let parsed = url::Url::parse(url);
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
#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_URL: &'static str =
        "https://api.example.com/twilio/conference_status.xml?waiter_id=42#rc=5&rp=all&sni=y";

    #[test]
    fn build_url_with_standard_port_test() {
        let url = url::Url::parse(REQUEST_URL).unwrap();
        let actual = build_url_with_standard_port(&url);
        let expected = "https://api.example.com:443/twilio/conference_status.xml?waiter_id=42#rc=5&rp=all&sni=y".to_string();
        assert_eq!(actual, expected);
    }
}
