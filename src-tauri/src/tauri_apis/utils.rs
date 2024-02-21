use anyhow::{anyhow, Result};
use entity::rules::{self, RuleAction, RuleType};
use entity::utils::get_random_port;
use kitty_proxy::TrafficStreamRule;
use reqwest;
use sea_orm::ConnectionTrait;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{self, Duration};
use tauri::utils::platform;
use tokio::task::JoinSet;

pub fn get_http_socks_ports(used_ports: &mut HashSet<u16>) -> (u16, u16) {
    let http_port = get_random_port(&used_ports).unwrap();
    let socks_port = get_random_port(&used_ports).unwrap();
    (http_port, socks_port)
}

pub fn relative_command_path(command: &Path) -> Result<PathBuf> {
    match platform::current_exe()?.parent() {
        #[cfg(windows)]
        Some(exe_dir) => Ok(exe_dir.join(command).with_extension("exe")),
        #[cfg(not(windows))]
        Some(exe_dir) => Ok(exe_dir.join(command)),
        None => Err(anyhow!("current exe not has parent.")),
    }
}

async fn request_test_url(port: u16, url: String) -> Result<(u16, Duration)> {
    let start_time = time::Instant::now();
    let proxy = reqwest::Proxy::http(format!("http://127.0.0.1:{port}"))?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .proxy(proxy)
        .build()?;
    let response = client.get(url).send().await;
    if response.is_err() {
        Ok((port, Duration::from_secs(3)))
    } else {
        let end_time = time::Instant::now();
        let delay = end_time - start_time;
        Ok((port, delay))
    }
}

pub async fn speed_delay(
    ports: Vec<u16>,
    test_url: Option<&str>,
) -> Result<HashMap<u16, Duration>> {
    let mut set = JoinSet::new();
    let url = test_url.unwrap_or("https://gstatic.com/generate_204");
    for port in ports.clone() {
        let url_clone = url.to_string().clone();
        set.spawn(async move { request_test_url(port, url_clone) });
    }
    let mut delay_dict: HashMap<u16, Duration> = ports
        .into_iter()
        .map(|x| (x, Duration::from_secs(3)))
        .collect();
    while let Some(res) = set.join_next().await {
        let aa = res?.await;
        if let Ok((port, delay)) = aa {
            delay_dict.insert(port, delay);
        }
    }
    Ok(delay_dict)
}

pub async fn add_rule2match_proxy(
    rwlock_share: &mut tokio::sync::RwLockWriteGuard<'_, kitty_proxy::MatchProxy>,
    rule_record: &rules::Model,
) {
    let traffic_stream_rule = match rule_record.rule_action {
        RuleAction::Reject => TrafficStreamRule::Reject,
        RuleAction::Direct => TrafficStreamRule::Direct,
        RuleAction::Proxy => TrafficStreamRule::Proxy,
    };
    match rule_record.rule_type {
        RuleType::Cidr => rwlock_share
            .add_cidr(rule_record.rule.as_str(), traffic_stream_rule)
            .unwrap(),
        RuleType::DomainPreffix => {
            rwlock_share.add_domain_preffix(rule_record.rule.clone(), traffic_stream_rule)
        }
        RuleType::DomainSuffix => {
            rwlock_share.add_domain_suffix(rule_record.rule.clone(), traffic_stream_rule)
        }
        RuleType::FullDomain => {
            rwlock_share.add_full_domain(rule_record.rule.clone(), traffic_stream_rule)
        }
        RuleType::DomainRoot => {
            rwlock_share.add_root_domain(rule_record.rule.as_str(), traffic_stream_rule)
        }
    }
}

pub async fn delete_rule2match_proxy<C>(
    db: &C,
    rwlock_share: &mut tokio::sync::RwLockWriteGuard<'_, kitty_proxy::MatchProxy>,
    rule_records: Vec<rules::Model>,
) -> Result<()>
where
    C: ConnectionTrait,
{
    if rule_records.is_empty() {
        return Ok(());
    }
    let has_direct = rule_records
        .iter()
        .any(|x| x.rule_action == RuleAction::Direct && x.rule_type == RuleType::Cidr);
    let has_not_direct = rule_records.iter().any(|x| {
        (x.rule_action == RuleAction::Proxy || x.rule_action == RuleAction::Reject)
            && x.rule_type == RuleType::Cidr
    });
    let cidr_rules = rules::Model::fetch_by_rule_type(db, RuleType::Cidr).await?;
    if has_direct {
        rwlock_share.reset_direct_cidr();
    }
    if has_not_direct {
        rwlock_share.clear_not_direct_cidr();
    }
    if has_not_direct || has_direct {
        for rule_record in cidr_rules {
            let traffic_stream_rule = match rule_record.rule_action {
                RuleAction::Reject => TrafficStreamRule::Reject,
                RuleAction::Direct => TrafficStreamRule::Direct,
                RuleAction::Proxy => TrafficStreamRule::Proxy,
            };
            rwlock_share.add_cidr(rule_record.rule.as_str(), traffic_stream_rule)?;
        }
    }
    let remain_delete_rule_records: Vec<rules::Model> = rule_records
        .iter()
        .filter(|&x| x.rule_type != RuleType::Cidr)
        .cloned()
        .collect();
    for rule_record in remain_delete_rule_records {
        match rule_record.rule_type {
            RuleType::DomainPreffix => {
                rwlock_share.delete_domain_preffix(rule_record.rule.as_str())
            }
            RuleType::DomainSuffix => rwlock_share.delete_domain_suffix(rule_record.rule.as_str()),
            RuleType::FullDomain => rwlock_share.delete_full_domain(rule_record.rule.as_str()),
            RuleType::DomainRoot => rwlock_share.delete_root_domain(rule_record.rule.as_str()),
            _ => (),
        }
    }
    Ok(())
}
