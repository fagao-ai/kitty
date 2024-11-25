use anyhow::Result;

#[cfg(target_os = "windows")]
static DEFAULT_BYPASS: &str = "localhost;127.*;192.168.*;<local>";
#[cfg(target_os = "linux")]
static DEFAULT_BYPASS: &str =
    "192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,127.0.0.1,localhost,*.local,::1";
#[cfg(target_os = "macos")]
static DEFAULT_BYPASS: &str = "192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,127.0.0.1,localhost,*.local,timestamp.apple.com,sequoia.apple.com,seed-sequoia.siri.apple.com";

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub fn set_system_proxy(host: &str, _socks_port: u16, http_port: Option<u16>) {
    use rustem_proxy::SystemProxy;
    // SystemProxy::set(SystemProxy {
    //     is_enabled: true,
    //     host: host.to_string(),
    //     port: _socks_port,
    //     bypass: DEFAULT_BYPASS.to_string(),
    //     protocol: rustem_proxy::Protocol::SOCKS,
    // });
    if http_port.is_some() {
        SystemProxy::set(SystemProxy {
            is_enabled: true,
            host: host.to_string(),
            port: http_port.unwrap(),
            bypass: DEFAULT_BYPASS.to_string(),
            protocol: rustem_proxy::Protocol::HTTP,
        });
        SystemProxy::set(SystemProxy {
            is_enabled: true,
            host: host.to_string(),
            port: http_port.unwrap(),
            bypass: DEFAULT_BYPASS.to_string(),
            protocol: rustem_proxy::Protocol::HTTPS,
        });
    }
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub fn clear_system_proxy() {
    use rustem_proxy::SystemProxy;
    SystemProxy::unset();
}
