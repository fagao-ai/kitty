use base64::{engine::general_purpose, DecodeError, Engine};
use entity::types::ProtocolLine;
use reqwest;
use anyhow::anyhow;

fn safe_decode_base64(text: &str, no_pad: bool) -> String {
    // let trimmed_text = text.trim();
    let bytes_data = if !no_pad {
        general_purpose::STANDARD.decode(text)
    } else {
        general_purpose::STANDARD_NO_PAD.decode(text)
    };

    match bytes_data {
        Ok(decode_bytes) => String::from_utf8(decode_bytes).expect("Invalid UTF-8 sequence"),
        Err(_e) => {
            match _e {
                DecodeError::InvalidPadding => safe_decode_base64(text, true),
                _ => text.to_string(),
            }
        },
    }
}

pub async fn download_subcriptions(url: &str) -> anyhow::Result<Vec<ProtocolLine>> {
    let client = reqwest::Client::builder()
            .user_agent("OKZTWO-Mac-Client-1.5.6")
            .build()?;

    let resp = client.get(url).send().await?;
    let resp_text = if resp.status().is_success() {
        resp.text().await?
    } else {
        return Err(anyhow!("download subscriptions failed.").into());
    };

    let decoded_text = safe_decode_base64(&resp_text, false);
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
    // println!("{:?}", results.len());
    anyhow::Ok(results)
}
