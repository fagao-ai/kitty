//! Unified server management using shoes library.
//!
//! This module provides a unified interface to start xray and hysteria2 servers
//! using the shoes library, instead of spawning separate processes.

use anyhow::{anyhow, Result};
use entity::{hysteria, xray};
use sea_orm::DatabaseConnection;
use shoes::config::Config;
use std::collections::HashSet;
use tauri::State;
use tokio::task::JoinHandle;

use crate::config_converter::ShoesConfigConverter;
use crate::state::{DatabaseState, ProcessManagerState};
use crate::types::{CommandResult, KittyResponse};

// Import start_servers from shoes library
use shoes::tcp::tcp_server::start_servers;

/// Server manager for handling multiple running servers.
pub struct ServerManager {
    /// Running server handles
    running_servers: Vec<JoinHandle<()>>,
    /// Used ports tracking
    used_ports: HashSet<u16>,
}

impl ServerManager {
    /// Create a new server manager.
    pub fn new() -> Self {
        Self {
            running_servers: Vec::new(),
            used_ports: HashSet::new(),
        }
    }

    /// Start servers based on current base config selection.
    ///
    /// This reads the base config to determine which server (xray or hysteria)
    /// to start, then launches the appropriate shoes server.
    ///
    /// Note: This is a simplified version that starts both xray and hysteria
    /// servers if they exist in the database.
    pub async fn start_servers_from_config(
        &mut self,
        db: &DatabaseConnection,
    ) -> Result<()> {
        // Stop any existing servers first
        self.stop_all_servers().await;

        // Try to start hysteria server if exists
        let hysteria_record = hysteria::Model::first(db).await?;
        if let Some(hysteria_record) = hysteria_record {
            self.start_hysteria_server(db, hysteria_record.id).await?;
        }

        // Try to start xray servers if any exist
        let xray_records = xray::Model::fetch_all(db).await?;
        if !xray_records.is_empty() {
            // Start the first xray server
            self.start_xray_server(db, xray_records[0].id).await?;
        }

        Ok(())
    }

    /// Start an xray server using shoes library.
    async fn start_xray_server(&mut self, db: &DatabaseConnection, xray_id: i32) -> Result<()> {
        let xray_record = xray::Model::get_by_id(db, xray_id)
            .await?
            .ok_or_else(|| anyhow!("Xray record {} not found", xray_id))?;

        let (http_port, socks_port) = self.get_available_ports()?;

        // Generate shoes YAML config for xray
        let yaml_config = ShoesConfigConverter::xray_to_socks_http_yaml(
            &xray_record,
            http_port,
            socks_port,
        )?;

        // Parse and start servers
        let configs: Vec<Config> = shoes::config::load_config_str(&yaml_config)?;

        for config in configs {
            let resolver = std::sync::Arc::new(shoes::resolver::CachingNativeResolver::new()) as std::sync::Arc<dyn shoes::resolver::Resolver>;
            let handles = start_servers(config, resolver).await?;
            self.running_servers.extend(handles);
        }

        Ok(())
    }

    /// Start a hysteria server using shoes library.
    async fn start_hysteria_server(
        &mut self,
        db: &DatabaseConnection,
        hysteria_id: i32,
    ) -> Result<()> {
        let hysteria_record = hysteria::Model::get_by_id(db, hysteria_id)
            .await?
            .ok_or_else(|| anyhow!("Hysteria record {} not found", hysteria_id))?;

        let (http_port, socks_port) = self.get_available_ports()?;

        // Generate shoes YAML config for hysteria
        let yaml_config = ShoesConfigConverter::hysteria_to_socks_http_yaml(
            &hysteria_record,
            http_port,
            socks_port,
        )?;

        // Parse and start servers
        let configs: Vec<Config> = shoes::config::load_config_str(&yaml_config)?;

        for config in configs {
            let resolver = std::sync::Arc::new(shoes::resolver::CachingNativeResolver::new()) as std::sync::Arc<dyn shoes::resolver::Resolver>;
            let handles = start_servers(config, resolver).await?;
            self.running_servers.extend(handles);
        }

        Ok(())
    }

    /// Stop all running servers.
    pub async fn stop_all_servers(&mut self) {
        for handle in self.running_servers.drain(..) {
            handle.abort();
        }
        self.used_ports.clear();
    }

    /// Get two available ports for HTTP and SOCKS5 proxies.
    fn get_available_ports(&mut self) -> Result<(u16, u16)> {
        let http_port = self.find_available_port()?;
        let socks_port = self.find_available_port()?;
        Ok((http_port, socks_port))
    }

    /// Find an available port that's not in use.
    fn find_available_port(&mut self) -> Result<u16> {
        // Try ports from 20000 to 30000
        for port in 20000..30000 {
            if !self.used_ports.contains(&port) {
                self.used_ports.insert(port);
                return Ok(port);
            }
        }
        Err(anyhow!("No available ports found"))
    }
}

impl Default for ServerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Start servers based on the current base config selection.
///
/// This is the main entry point for starting servers from Tauri commands.
pub async fn start_servers_from_db(
    process_manager: &ProcessManagerState,
    db: &DatabaseConnection,
) -> Result<()> {
    let _manager = process_manager.running_servers.lock().await;

    // Create a temporary server manager to handle the startup
    let mut server_manager = ServerManager::new();
    server_manager.start_servers_from_config(db).await?;

    // Transfer the running servers to the process manager state
    let mut state_guard = process_manager.running_servers.lock().await;
    *state_guard = server_manager.running_servers;

    Ok(())
}

/// Stop all currently running servers.
pub async fn stop_all_servers(process_manager: &ProcessManagerState) -> Result<()> {
    let mut servers = process_manager.running_servers.lock().await;
    for handle in servers.drain(..) {
        handle.abort();
    }
    println!("All servers stopped");
    Ok(())
}

/// Check if any servers are currently running.
pub async fn is_any_server_running(process_manager: &ProcessManagerState) -> bool {
    let servers = process_manager.running_servers.lock().await;
    !servers.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_port() {
        let mut manager = ServerManager::new();
        let port1 = manager.find_available_port().unwrap();
        let port2 = manager.find_available_port().unwrap();
        assert_ne!(port1, port2);
        assert!(manager.used_ports.contains(&port1));
        assert!(manager.used_ports.contains(&port2));
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Start the configured proxy server based on base_config.
///
/// This command reads the base_config to determine which server (xray or hysteria)
/// to start, then launches the appropriate shoes server.
#[tauri::command(rename_all = "snake_case")]
pub async fn start_proxy_server<'a>(
    state: State<'a, DatabaseState>,
    process_manager: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    start_servers_from_db(&process_manager, &db).await?;
    Ok(KittyResponse::default())
}

/// Stop all currently running proxy servers.
#[tauri::command(rename_all = "snake_case")]
pub async fn stop_proxy_server<'a>(
    process_manager: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<()>> {
    stop_all_servers(&process_manager).await?;
    Ok(KittyResponse::default())
}

/// Check if any proxy server is currently running.
#[tauri::command(rename_all = "snake_case")]
pub async fn is_proxy_server_running<'a>(
    process_manager: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<bool>> {
    let running = is_any_server_running(&process_manager).await;
    Ok(KittyResponse::from_data(running))
}
