use anyhow::Result;
use sysproxy::Sysproxy;

#[cfg(target_os = "windows")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) {
    let mut socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: "localhost;127.*".into(),
        #[cfg(not(target_os = "windows"))]
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_system_proxy();
}

#[cfg(target_os = "linux")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) -> Result<()> {
    let mut socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_enable()?;
    let _ = socks_sysproxy.set_socks();
    match http_port {
        Some(http_port) => {
            let mut socks_sysproxy = Sysproxy {
                enable: true,
                host: host.into(),
                port: http_port,
                bypass: "localhost,127.0.0.1/8".into(),
            };
            let _ = socks_sysproxy.set_http();
            let _ = socks_sysproxy.set_https();
        }
        None => (),
    }
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) {
    let mut socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_socks();
    match http_port {
        Some(http_port) => {
            let mut socks_sysproxy = Sysproxy {
                enable: true,
                host: host.into(),
                port: http_port,
                bypass: "localhost,127.0.0.1/8".into(),
            };
            let _ = socks_sysproxy.set_http();
            let _ = socks_sysproxy.set_https();
        }
        None => (),
    }
}

#[cfg(target_os = "windows")]
pub fn clear_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) {
    let mut socks_sysproxy = Sysproxy {
        enable: false,
        host: host.into(),
        port: socks_port,
        bypass: "localhost;127.*".into(),
        #[cfg(not(target_os = "windows"))]
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_system_proxy();
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
pub fn clear_system_proxy() {
    use std::process::Command;
    Command::new("networksetup")
        .args([target_state.as_str(), service, enable])
        .status()?;
    Ok(())
}
