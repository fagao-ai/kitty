use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HyTls {
    sni: String,
    insecure: bool,
    #[serde(rename = "pinSHA256")] 
    pin_sha256: Option<String>,
    ca: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct HyBandwidth {
    up: String,
    down: String,
}
#[derive(Serialize, Deserialize)]
struct HySock5 {
    listen: String,
}
#[derive(Serialize, Deserialize)]
struct HyHttp {
    listen: String,
}

#[derive(Serialize, Deserialize)]
pub struct HyConfig {
    server: String,
    auth: String,
    tls: HyTls,
    bandwidth: HyBandwidth,
    socks5: HySock5,
    http: HyHttp,
}
