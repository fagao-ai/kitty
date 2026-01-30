//! Auto-starter module for measuring proxy delays and starting the fastest server.
//!
//! This module provides functionality to automatically test proxy delays
//! and start the fastest server on app startup.

use anyhow::{anyhow, Result};
use entity::{base_config, hysteria, xray};
use log::{info, warn};
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::config_converter::ShoesConfigConverter;
use crate::proxy::delay::{test_all_proxies_delay, ProxyType};
use crate::state::ProcessManagerState;
use crate::tauri_apis::start_servers_internal;

/// Result of auto-start operation.
#[derive(Debug, Clone)]
pub enum AutoStartResult {
    Success {
        #[allow(dead_code)]
        proxy_id: u32,
        #[allow(dead_code)]
        proxy_type: ProxyType,
        #[allow(dead_code)]
        delay: u128,
    },
    Fallback {
        #[allow(dead_code)]
        proxy_id: u32,
        #[allow(dead_code)]
        proxy_type: ProxyType,
    },
    NoProxies,
    AlreadyRunning,
}

/// Auto-starter for measuring delays and starting the fastest proxy server.
pub struct AutoStarter {
    db: DatabaseConnection,
    process_manager: ProcessManagerState,
    resource_dir: PathBuf,
    custom_rules_path: PathBuf,
    is_running: std::sync::Arc<AtomicBool>,
}

impl AutoStarter {
    /// Create a new auto-starter instance.
    pub fn new(
        db: DatabaseConnection,
        process_manager: ProcessManagerState,
        resource_dir: PathBuf,
        custom_rules_path: PathBuf,
    ) -> Self {
        Self {
            db,
            process_manager,
            resource_dir,
            custom_rules_path,
            is_running: std::sync::Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the fastest server based on delay measurements.
    ///
    /// This function:
    /// 1. Fetches all configured proxies
    /// 2. Measures delays for all proxies
    /// 3. Selects the fastest one
    /// 4. Starts the selected server (without setting system proxy)
    pub async fn start_fastest_server(&self) -> Result<AutoStartResult> {
        // Prevent concurrent auto-starts
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(AutoStartResult::AlreadyRunning);
        }

        self.is_running.store(true, Ordering::SeqCst);
        let _guard = scopeguard::guard(self.is_running.clone());

        info!("Starting auto-measure and fastest server selection");

        // Fetch all proxies
        let xray_records = xray::Model::fetch_all(&self.db).await?;
        let hysteria_record = hysteria::Model::first(&self.db).await?;

        let hysteria_records = if let Some(record) = hysteria_record {
            vec![record]
        } else {
            vec![]
        };

        // Check if any proxies exist
        if xray_records.is_empty() && hysteria_records.is_empty() {
            info!("No proxies configured, skipping auto-start");
            return Ok(AutoStartResult::NoProxies);
        }

        info!(
            "Testing {} xray and {} hysteria proxies",
            xray_records.len(),
            hysteria_records.len()
        );

        // Run delay tests
        let results = test_all_proxies_delay(xray_records, hysteria_records).await;

        // Filter out timeouts
        let valid_results: Vec<_> = results.iter().filter(|r| r.delay < 9999).collect();

        if valid_results.is_empty() {
            warn!("All delay tests timed out, using fallback");
            return self.fallback_to_first_proxy().await;
        }

        // Select fastest
        let fastest = valid_results[0];
        info!(
            "Fastest proxy: ID={:?}, type={:?}, delay={}ms",
            fastest.id, fastest.proxy_type, fastest.delay
        );

        // Start the fastest server
        self.start_server_by_id(fastest.id, fastest.proxy_type).await?;

        Ok(AutoStartResult::Success {
            proxy_id: fastest.id,
            proxy_type: fastest.proxy_type,
            delay: fastest.delay,
        })
    }

    /// Start a server by its ID and type.
    async fn start_server_by_id(&self, id: u32, proxy_type: ProxyType) -> Result<()> {
        let yaml_config = match proxy_type {
            ProxyType::Xray => {
                let xray_record = xray::Model::get_by_id(&self.db, id as i32)
                    .await?
                    .ok_or_else(|| anyhow!("Xray record {} not found", id))?;

                // Get ports from base config
                let base_config = base_config::Model::first(&self.db)
                    .await?
                    .ok_or_else(|| anyhow!("Base config not found"))?;

                let yaml_config = ShoesConfigConverter::xray_to_socks_http_yaml(
                    &xray_record,
                    base_config.http_port,
                    base_config.socks_port,
                    &self.resource_dir,
                    Some(&self.custom_rules_path),
                )?;

                // Print YAML config for debugging
                println!("=== Auto-start Xray YAML config ===");
                println!("{}", yaml_config);
                println!("=== End of YAML config ===");

                yaml_config
            }
            ProxyType::Hysteria2 => {
                let hysteria_record = hysteria::Model::get_by_id(&self.db, id as i32)
                    .await?
                    .ok_or_else(|| anyhow!("Hysteria record {} not found", id))?;

                let base_config = base_config::Model::first(&self.db)
                    .await?
                    .ok_or_else(|| anyhow!("Base config not found"))?;

                let yaml_config = ShoesConfigConverter::hysteria_to_socks_http_yaml(
                    &hysteria_record,
                    base_config.http_port,
                    base_config.socks_port,
                    &self.resource_dir,
                    Some(&self.custom_rules_path),
                )?;

                // Print YAML config for debugging
                println!("=== Auto-start Hysteria YAML config ===");
                println!("{}", yaml_config);
                println!("=== End of YAML config ===");

                yaml_config
            }
        };

        // Parse and start servers
        info!("About to load shoes config and start servers");

        let configs = shoes::config::load_config_str(&yaml_config)?;
        let mut all_handles = Vec::new();

        for config in configs {
            let handles = start_servers_internal(config).await?;
            all_handles.extend(handles);
        }

        // Store handles in process manager
        let mut running_servers = self.process_manager.running_servers.lock().await;
        running_servers.extend(all_handles);

        // Record active proxy info
        *self.process_manager.active_proxy_id.lock().await = Some(id);
        *self.process_manager.active_proxy_type.lock().await = Some(proxy_type.to_string());

        info!(
            "Successfully started server: ID={:?}, type={:?}",
            id, proxy_type
        );
        Ok(())
    }

    /// Fallback to the first available proxy when all delay tests fail.
    async fn fallback_to_first_proxy(&self) -> Result<AutoStartResult> {
        // Try hysteria first
        if let Ok(Some(hysteria_record)) = hysteria::Model::first(&self.db).await {
            info!("Using fallback: hysteria proxy ID={}", hysteria_record.id);
            self.start_server_by_id(hysteria_record.id as u32, ProxyType::Hysteria2)
                .await?;
            return Ok(AutoStartResult::Fallback {
                proxy_id: hysteria_record.id as u32,
                proxy_type: ProxyType::Hysteria2,
            });
        }

        // Then try xray
        let xray_records = xray::Model::fetch_all(&self.db).await?;
        if !xray_records.is_empty() {
            let first = &xray_records[0];
            info!("Using fallback: xray proxy ID={}", first.id);
            self.start_server_by_id(first.id as u32, ProxyType::Xray)
                .await?;
            return Ok(AutoStartResult::Fallback {
                proxy_id: first.id as u32,
                proxy_type: ProxyType::Xray,
            });
        }

        Ok(AutoStartResult::NoProxies)
    }
}

/// Simple scope guard implementation for cleanup.
mod scopeguard {
    use std::sync::atomic::{AtomicBool, Ordering};

    pub struct Guard {
        is_running: std::sync::Arc<AtomicBool>,
    }

    impl Drop for Guard {
        fn drop(&mut self) {
            self.is_running.store(false, Ordering::SeqCst);
        }
    }

    pub fn guard(is_running: std::sync::Arc<AtomicBool>) -> Guard {
        Guard { is_running }
    }
}
