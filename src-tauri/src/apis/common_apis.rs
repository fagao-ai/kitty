use crate::rules::{read_rules_file, write_rules_file, Rule, RuleType as FileRuleType};
use crate::types::{CommandResult, KittyResponse};
use anyhow::Result;
use entity::{base_config, rules};
use sea_orm::ConnectionTrait;
use std::path::PathBuf;

pub struct CommonAPI;

impl CommonAPI {
    /// Migrate rules from database to file
    /// This should be called once during startup to migrate existing rules
    pub async fn migrate_rules_from_db_to_file<C>(db: &C, rules_path: PathBuf) -> Result<()>
    where
        C: ConnectionTrait,
    {
        // Check if file already exists
        if rules_path.exists() {
            log::info!("Rules file already exists at {}, skipping migration", rules_path.display());
            return Ok(());
        }

        // Fetch rules from database
        let db_rules = rules::Model::fetch_all(db).await?;
        if db_rules.is_empty() {
            log::info!("No rules found in database, nothing to migrate");
            return Ok(());
        }

        // Convert to file format
        let file_rules: Vec<Rule> = db_rules
            .into_iter()
            .map(|r| Rule {
                pattern: r.rule,
                rule_type: match r.rule_type {
                    rules::RuleType::DomainSuffix => FileRuleType::DomainSuffix,
                    rules::RuleType::DomainPreffix => FileRuleType::DomainPrefix,
                    rules::RuleType::FullDomain => FileRuleType::FullDomain,
                    rules::RuleType::Cidr => FileRuleType::Cidr,
                    rules::RuleType::DomainRoot => FileRuleType::DomainRoot,
                },
                action: match r.rule_action {
                    rules::RuleAction::Proxy => crate::rules::RuleAction::Proxy,
                    rules::RuleAction::Direct => crate::rules::RuleAction::Direct,
                    rules::RuleAction::Reject => crate::rules::RuleAction::Reject,
                },
            })
            .collect();

        // Write to file
        write_rules_file(&rules_path, &file_rules)?;
        log::info!(
            "Migrated {} rules from database to {}",
            file_rules.len(),
            rules_path.display()
        );

        Ok(())
    }
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    pub async fn copy_proxy_env<C>(db: &C) -> Result<String>
    where
        C: ConnectionTrait,
    {
        let record = base_config::Model::first(db).await?.ok_or_else(|| anyhow::anyhow!("base_config not exists"))?;
        let http_port = record.http_port;
        let socks_port = record.socks_port;
        #[cfg(target_os = "windows")]
            let env_expr = format!("set https_proxy=http://127.0.0.1:{http_port} http_proxy=http://127.0.0.1:{http_port} all_proxy=socks5://127.0.0.1:{socks_port}");

        #[cfg(any(target_os = "macos", target_os = "linux"))]
            let env_expr = format!("export https_proxy=http://127.0.0.1:{http_port} http_proxy=http://127.0.0.1:{http_port} all_proxy=socks5://127.0.0.1:{socks_port}");

        Ok(env_expr)
    }

    pub async fn query_base_config<C>(db: &C) -> CommandResult<KittyResponse<base_config::Model>>
    where
        C: ConnectionTrait,
    {
        let record = base_config::Model::first(db).await?;
        let response = match record {
            Some(record) => KittyResponse::<base_config::Model>::from_data(record),
            None => {
                // Create default base_config if not exists
                let default_config = base_config::Model {
                    id: 0,
                    local_ip: "127.0.0.1".to_string(),
                    http_port: 10086,
                    socks_port: 10087,
                    delay_test_url: "https://gstatic.com/generate_204".to_string(),
                    sysproxy_flag: false,
                    auto_start: false,
                    language: "zh-CN".to_string(),
                    allow_lan: false,
                    mode: "Rules".to_string(),
                    update_interval: 3,
                    log_level: "debug".to_string(),
                };
                let inserted = default_config.insert_one(db).await?;
                KittyResponse::<base_config::Model>::from_data(inserted)
            }
        };
        Ok(response)
    }

    pub async fn update_base_config<C>(
        db: &C,
        record: base_config::Model,
    ) -> CommandResult<KittyResponse<base_config::Model>>
    where
        C: ConnectionTrait,
    {
        let updated_record = record.update(db).await?;
        Ok(KittyResponse::<base_config::Model>::from_data(
            updated_record,
        ))
    }

    pub async fn add_rules(rules_path: PathBuf, records: Vec<Rule>) -> CommandResult<KittyResponse<()>> {
        let mut existing_rules = read_rules_file(&rules_path).unwrap_or_default();
        existing_rules.extend(records);
        write_rules_file(&rules_path, &existing_rules)?;
        Ok(KittyResponse::default())
    }

    pub async fn query_rules(rules_path: PathBuf) -> CommandResult<KittyResponse<Vec<Rule>>> {
        let rules = read_rules_file(&rules_path).unwrap_or_default();
        Ok(KittyResponse::from_data(rules))
    }

    pub async fn delete_rules(rules_path: PathBuf, ids: Vec<usize>) -> CommandResult<KittyResponse<()>> {
        let mut rules = read_rules_file(&rules_path).unwrap_or_default();
        // Remove rules by index (ids are 1-based from frontend)
        // Sort in descending order to avoid index shifting issues
        let mut sorted_ids = ids;
        sorted_ids.sort_by(|a, b| b.cmp(a));
        for idx in sorted_ids {
            if idx > 0 && idx <= rules.len() {
                rules.remove(idx - 1);
            }
        }
        write_rules_file(&rules_path, &rules)?;
        Ok(KittyResponse::default())
    }
}
