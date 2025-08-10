use std::collections::VecDeque;
use tokio::time::{sleep, Duration};
use crate::Result;

pub struct ProxyManager {
    proxies: VecDeque<String>,
    current_proxy: Option<String>,
    rotation_interval: Duration,
}

impl ProxyManager {
    pub fn new(proxies: Vec<String>) -> Self {
        let mut proxy_queue = VecDeque::new();
        for proxy in proxies {
            proxy_queue.push_back(proxy);
        }

        Self {
            proxies: proxy_queue,
            current_proxy: None,
            rotation_interval: Duration::from_secs(60),
        }
    }

    pub fn get_current_proxy(&self) -> Option<&str> {
        self.current_proxy.as_deref()
    }

    pub fn rotate_proxy(&mut self) -> Option<String> {
        if let Some(proxy) = self.proxies.pop_front() {
            self.proxies.push_back(proxy.clone());
            self.current_proxy = Some(proxy.clone());
            Some(proxy)
        } else {
            None
        }
    }

    pub async fn auto_rotate(&mut self) {
        loop {
            sleep(self.rotation_interval).await;
            self.rotate_proxy();
        }
    }

    pub fn set_rotation_interval(&mut self, interval: Duration) {
        self.rotation_interval = interval;
    }

    pub fn add_proxy(&mut self, proxy: String) {
        self.proxies.push_back(proxy);
    }

    pub fn remove_proxy(&mut self, proxy: &str) {
        self.proxies.retain(|p| p != proxy);
    }
}
