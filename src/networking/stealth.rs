use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use rand::Rng;
use crate::Result;

pub struct StealthMode {
    enabled: bool,
    delay_range: (u64, u64), // min, max delay in seconds
    user_agents: Vec<String>,
    headers: HashMap<String, Vec<String>>,
}

impl StealthMode {
    pub fn new() -> Self {
        let user_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
        ];

        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), vec![
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".to_string(),
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
        ]);
        headers.insert("Accept-Language".to_string(), vec![
            "en-US,en;q=0.5".to_string(),
            "en-GB,en;q=0.9".to_string(),
        ]);
        headers.insert("Accept-Encoding".to_string(), vec![
            "gzip, deflate, br".to_string(),
        ]);

        Self {
            enabled: false,
            delay_range: (1, 5),
            user_agents,
            headers,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_delay_range(&mut self, min: u64, max: u64) {
        self.delay_range = (min, max);
    }

    pub async fn random_delay(&self) {
        if !self.enabled {
            return;
        }

        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(self.delay_range.0..=self.delay_range.1);
        sleep(Duration::from_secs(delay)).await;
    }

    pub fn get_random_user_agent(&self) -> &str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.user_agents.len());
        &self.user_agents[index]
    }

    pub fn get_random_header(&self, header_name: &str) -> Option<&str> {
        if let Some(values) = self.headers.get(header_name) {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..values.len());
            Some(&values[index])
        } else {
            None
        }
    }

    pub fn add_user_agent(&mut self, user_agent: String) {
        self.user_agents.push(user_agent);
    }

    pub fn add_header_value(&mut self, header: String, value: String) {
        self.headers.entry(header).or_insert_with(Vec::new).push(value);
    }
}
