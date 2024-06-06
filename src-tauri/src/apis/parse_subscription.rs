use base64::{engine::general_purpose, Engine};
use entity::types::ProtocolLine;
use reqwest;


pub async fn download_subcriptions(url: &str) -> anyhow::Result<Vec<ProtocolLine>> {
    let resp = reqwest::get(url).await?;
    let resp_text = resp.text().await?;
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
            let trimed_line = if let Some(pos) = line.rfind('#') {
                &line[..pos]
            } else {
                line
            };
            let protocol = trimed_line.split("://").next().unwrap();
            // let protocol_str = trimed_line.split("://").last().unwrap();
            results.push(ProtocolLine::new(trimed_line.to_string(), protocol.into()))
            // if protocol_str.starts_with("eyJ") {
            //     // let decode_bytes = general_purpose::STANDARD.decode(new_protocol_str)?;
            //     // let protocol_line =String::from_utf8(decode_bytes).expect("Invalid UTF-8 sequence");
            //     let new_line = format!("{protocol}://{protocol_str}");
            //     println!("new_line: {}", new_line);
            //     results.push(ProtocolLine::new(new_line, protocol.into()))
            // }
        }
    }
    println!("{}", results.len());
    anyhow::Ok(results)
}
