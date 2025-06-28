use url::Url;

pub fn builder(base_url: &str, params: Vec<(&str, &str)>) -> String {
    let mut url = Url::parse(base_url).expect("Unable to parse base URL");

    url.query_pairs_mut().clear();
    for (key, value) in params {
        url.query_pairs_mut().append_pair(key, value);
    }

    url.to_string()
}
