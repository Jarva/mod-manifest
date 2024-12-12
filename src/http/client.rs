use reqwest::{header, Client};

pub fn get_cf_client() -> Client {
    let mut headers = header::HeaderMap::new();
    let mut api_key = header::HeaderValue::from_str(
        dotenvy::var("CF_API_KEY")
            .expect("CF_API_KEY not set")
            .as_str(),
    ).unwrap();

    api_key.set_sensitive(true);

    headers.insert("x-api-key", api_key);

    Client::builder().default_headers(headers).build().expect("Unable to build http client")
}
