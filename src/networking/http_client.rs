use reqwest::{Client, Response, RequestBuilder};
use std::collections::HashMap;
use std::time::Duration;
use crate::Result;

pub struct HttpClient {
    client: Client,
    headers: HashMap<String, String>,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), 
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string());

        Self { client, headers }
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        let mut request = self.client.get(url);
        
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        Ok(response)
    }

    pub async fn get_with_proxy(&self, url: &str, proxy: &str) -> Result<Response> {
        let proxy = reqwest::Proxy::all(proxy)?;
        let client = Client::builder()
            .proxy(proxy)
            .timeout(Duration::from_secs(30))
            .build()?;

        let mut request = client.get(url);
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        Ok(response)
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}
