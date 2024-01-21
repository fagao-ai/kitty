use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyInfo {
    pub proxy: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyDelay {
    pub id: u32,
    pub delay: u128,
}

pub async fn kitty_proxies_delay(proxies: Vec<ProxyInfo>) -> Vec<ProxyDelay> {
    let mut result = Vec::new();
    for proxy in proxies.iter() {
        let client = Client::builder()
            .proxy(Proxy::all(proxy.proxy.clone()).unwrap())
            .build()
            .unwrap();
        let start_time = Instant::now();
        let response = client.get("https://google.com").send().await;
        match response {
            Ok(_) => {
                let elapsed = start_time.elapsed();
                println!("{}ms", elapsed.as_millis());
                let proxy_delay = ProxyDelay {
                    id: proxy.id,
                    delay: elapsed.as_millis(),
                };
                result.push(proxy_delay);
            }
            Err(_) => {
                let proxy_delay = ProxyDelay {
                    id: proxy.id,
                    delay: 9999,
                };
                result.push(proxy_delay);
            }
        };
    }
    // sory result by delay
    result.sort_by(|a, b| a.delay.cmp(&b.delay));
    result
}

#[cfg(test)]
#[tokio::test]
async fn test_proxies() {
    let proxies = vec![ProxyInfo {
        id: 1,
        proxy: "http://127.0.0.1:7890".to_string(),
    }];

    let results = kitty_proxies_delay(proxies).await;

    assert!(results.len() > 0);
    assert!(results[0].delay > 0);
}
