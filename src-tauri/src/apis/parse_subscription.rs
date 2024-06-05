use base64::{engine::general_purpose, Engine};
use reqwest;

const XRAY_SCHEMAS: [&str; 3] = ["vmess", "vless", "trojan"];
const HYSTERIA_SCHEMAS: [&str; 1] = ["hy2"];

pub struct ProtocolLine{
    pub line: String,
    pub protocol: String,
}

impl ProtocolLine {
    fn new(line: String, protocol: String) -> Self {
        Self {
            line,
            protocol
        }
    }

    pub fn is_xray(&self) -> bool{
        XRAY_SCHEMAS.contains(&&self.protocol.as_str())
    }

    pub fn is_hy2(&self) -> bool{
        HYSTERIA_SCHEMAS.contains(&&self.protocol.as_str())
    }
}

pub async fn download_subcriptions(url: &str) -> anyhow::Result<Vec<ProtocolLine>> {
    println!("download subscriptions");
    let resp = reqwest::get(url).await?;
    let resp_text = resp.text().await?;
    println!("resp_text");
    let decode_bytes_res = general_purpose::STANDARD.decode(&resp_text);
    let decoded_text = match decode_bytes_res {
        Ok(decode_bytes) => String::from_utf8(decode_bytes).expect("Invalid UTF-8 sequence"),
        Err(_e) => resp_text,
    };
    let mut results = Vec::new();
    for line in decoded_text.lines() {
        let line = line.trim();
        if line.trim().starts_with("#") {
            continue;
        }
        if line.contains("://") {
            let protocol = line.split("://").next().unwrap();
            let protocol_str = line.split("://").last().unwrap();
            if protocol_str.starts_with("eyJ") {
                let new_protocol_str = if let Some(pos) = protocol_str.find('#') {
                    &protocol_str[..pos]
                } else {
                    protocol_str
                };
                println!("new_protocol_str: {}", new_protocol_str);
                let decode_bytes = general_purpose::STANDARD.decode(new_protocol_str)?;
                let protocol_line =String::from_utf8(decode_bytes).expect("Invalid UTF-8 sequence");
                results.push(ProtocolLine::new(protocol_line, protocol.into()))
            }
        }
    }
    println!("{}", results.len());
    anyhow::Ok(results)
}
