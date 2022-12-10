use url::Url;

mod hash;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn build_url_with_standard_port(url: Url) -> String {
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

pub fn add_port(url: Url) -> String {
    if let Some(_) = url.port() {
        return url.to_string();
    }
    build_url_with_standard_port(url)
}

pub fn remove_port(url: Url) -> String {
    let mut new_url = url.clone();
    new_url.set_port(None).unwrap();
    new_url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_URL: &'static str =
        "https://api.example.com/twilio/conference_status.xml?waiter_id=42#rc=5&rp=all&sni=y";

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn build_url_with_standard_port_test() {
        let url = url::Url::parse(REQUEST_URL).unwrap();
        let actual = build_url_with_standard_port(url);
        let expected = "https://api.example.com:443/twilio/conference_status.xml?waiter_id=42#rc=5&rp=all&sni=y".to_string();
        assert_eq!(actual, expected);
    }
}
