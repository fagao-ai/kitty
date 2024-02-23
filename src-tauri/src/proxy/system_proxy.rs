use anyhow::Result;
use sysproxy::Sysproxy;


#[cfg(target_os = "windows")]
static DEFAULT_BYPASS: &str = "localhost;127.*;192.168.*;<local>";
#[cfg(target_os = "linux")]
static DEFAULT_BYPASS: &str = "192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,127.0.0.1,localhost,*.local,::1";
#[cfg(target_os = "macos")]
static DEFAULT_BYPASS: &str = "192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,127.0.0.1,localhost,*.local,timestamp.apple.com,sequoia.apple.com,seed-sequoia.siri.apple.com";

#[cfg(target_os = "windows")]
pub fn set_system_proxy(host: &str, _socks_port: u16, http_port: Option<u16>) -> Result<()> {
    use anyhow::anyhow;

    let res = if let Some(port) = http_port {
        let socks_sysproxy = Sysproxy {
            enable: true,
            host: host.into(),
            port: port,
            bypass: DEFAULT_BYPASS.into(),
        };

        let _ = socks_sysproxy.set_system_proxy();
        Ok(())
    } else {
        Err(anyhow!("windows must set http proxy port"))
    };
    res
}

#[cfg(target_os = "linux")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) -> Result<()> {
    let socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: DEFAULT_BYPASS.into(),
    };
    let _ = socks_sysproxy.set_enable()?;
    let _ = socks_sysproxy.set_socks();
    match http_port {
        Some(http_port) => {
            let socks_sysproxy = Sysproxy {
                enable: true,
                host: host.into(),
                port: http_port,
                bypass: DEFAULT_BYPASS.into(),
            };
            let _ = socks_sysproxy.set_http();
            let _ = socks_sysproxy.set_https();
        }
        None => (),
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn get_active_network_interface() -> Result<String> {
    use anyhow::anyhow;
    use std::net::Ipv4Addr;
    use std::str::FromStr;
    use std::{collections::HashMap, process::Command};
    // networksetup -listallnetworkservices
    let output = Command::new("networksetup")
        .arg("-listallnetworkservices")
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines = stdout.lines();
        let lines: Vec<&str> = lines.filter(|x| !x.contains("asterisk (*)")).collect();
        for service in lines {
            let info = Command::new("networksetup")
                .arg("-getinfo")
                .arg(service.trim())
                .output()?;
            if info.status.success() {
                let mut info_map: HashMap<String, String> = HashMap::new();
                let stdout = String::from_utf8_lossy(&info.stdout);
                let lines = stdout.lines();
                for line_item in lines {
                    if line_item.contains(":") {
                        let mut parts = line_item.trim().splitn(2, ':').map(str::trim);
                        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                            info_map.insert(key.to_string(), value.to_string());
                        }
                    }
                }
                if let Some(value) = info_map.get("IP address") {
                    match Ipv4Addr::from_str(value) {
                        Ok(_) => {
                            return Ok(service.to_owned().to_string());
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    }
    Err(anyhow!("Not found aviliable network interface."))
}

#[cfg(target_os = "macos")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) -> Result<&str> {
    use anyhow::Ok;

    let service = get_active_network_interface()?;
    let socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: DEFAULT_BYPASS.into(),
    };
    let _ = socks_sysproxy.set_socks(service.as_str());
    match http_port {
        Some(http_port) => {
            let socks_sysproxy = Sysproxy {
                enable: true,
                host: host.into(),
                port: http_port,
                bypass: DEFAULT_BYPASS.into(),
            };
            let _ = socks_sysproxy.set_http(service.as_str());
            let _ = socks_sysproxy.set_https(service.as_str());
            Ok("set socks proxy success")
        }
        None => Ok("the http_port is not set"),
    }
}

#[cfg(target_os = "windows")]
pub fn clear_system_proxy() -> Result<()>{
    let socks_sysproxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 10086,
        bypass: DEFAULT_BYPASS.into(),
    };
    let _ = socks_sysproxy.set_system_proxy();
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn clear_system_proxy() -> Result<()> {
    use std::process::Command;
    const CMD_KEY: &str = "org.gnome.system.proxy";

    Command::new("gsettings")
        .args(["set", CMD_KEY, "mode", "none"])
        .status()?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn clear_system_proxy() -> Result<()> {
    let socks_sysproxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 10086,
        bypass: DEFAULT_BYPASS.into(),
    };

    let _ = socks_sysproxy.set_system_proxy();

    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub fn has_sys_proxy() -> Result<bool> {
    let sys_proxy = Sysproxy::get_system_proxy()?;
    Ok(sys_proxy.enable)
}
