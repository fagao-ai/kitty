use entity::xray;
use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyInfo {
    pub address: String,
    pub port: u16,
    pub id: u32,
}

impl From<xray::Model> for ProxyInfo {
    fn from(source: xray::Model) -> Self {
        return ProxyInfo {
            id: source.id as u32,
            address: source.address,
            port: source.port,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyDelay {
    pub id: u32,
    pub delay: u128,
}

async fn measure_tcp_latency(proxy_info: &ProxyInfo) -> ProxyDelay {
    let address = format!("{}:{}", proxy_info.address, proxy_info.port);

    // 记录开始时间
    let start_time = Instant::now();

    // 尝试连接到目标 IP 和端口
    match tokio::time::timeout(
        std::time::Duration::from_secs(3),
        TcpStream::connect(address),
    )
    .await
    {
        Ok(_) => {
            // 计算往返时间
            let round_trip_time = start_time.elapsed();
            let proxy_delay = ProxyDelay {
                id: proxy_info.id,
                delay: round_trip_time.as_millis(),
            };
            return proxy_delay;
        }
        Err(_) => {
            let proxy_delay = ProxyDelay {
                id: proxy_info.id,
                delay: 9999,
            };
            return proxy_delay;
        }
    }
}

pub async fn kitty_proxies_delay(proxies: Vec<ProxyInfo>) -> Vec<ProxyDelay> {
    let mut result = Vec::new();
    let max_concurrent_connections = 10;

    let seamphore = Arc::new(Semaphore::new(max_concurrent_connections));

    let mut handles = vec![];
    for proxy in proxies.into_iter() {
        let permit = seamphore.clone().acquire_owned().await.unwrap();
        handles.push(tokio::spawn(async move {
            let _permit = permit;
            measure_tcp_latency(&proxy).await
        }));
    }

    for handle in handles {
        let res = handle.await.unwrap();
        result.push(res);
    }

    // sory result by delay
    result.sort_by(|a, b| a.delay.cmp(&b.delay));
    result
}

pub async fn kitty_current_proxy_delay(proxy: String, target_url: String) -> u128 {
    let request = Client::builder()
        .proxy(Proxy::all(proxy).unwrap())
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();
    let start_time = Instant::now();
    match request.get(target_url).send().await {
        Ok(_) => start_time.elapsed().as_millis(),
        Err(_) => 9999_u128,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proxies() {
        let proxies = vec![
            ProxyInfo {
                id: 1,
                address: "xj0211.alibabaokz.com".to_string(),
                port: 40001,
            },
            ProxyInfo {
                id: 2,
                address: "hk0106.alibabaokz.com".to_string(),
                port: 60126,
            },
        ];

        let mut aa = Vec::new();

        for _ in 0..100 {
            aa.extend(proxies.clone().into_iter());
        }

        let results = kitty_proxies_delay(aa).await;
        println!("{:?}", results);
        assert!(results.len() > 0);
        assert!(results[0].delay > 0);
    }

    #[tokio::test]
    async fn test_current_proxy() {
        let delay = kitty_current_proxy_delay(
            "http://127.0.0.1:7890".to_string(),
            "https://www.google.com".to_string(),
        )
        .await;

        println!("delay {}", delay);
    }
}
