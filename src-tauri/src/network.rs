use local_ip_address::list_afinet_netifas;
use std::net::Ipv4Addr;

fn is_physical_interface(name: &str) -> bool {
    let n = name.to_lowercase();
    n.starts_with("en")
        || n.starts_with("eth")
        || n.starts_with("wlan")
        || n.starts_with("wlp")
        || n.starts_with("enp")
        || n.starts_with("ens")
        || n == "wi-fi"
        || n == "ethernet"
}

fn is_virtual_interface(name: &str) -> bool {
    let n = name.to_lowercase();
    n.starts_with("utun")
        || n.starts_with("awdl")
        || n.starts_with("llw")
        || n.starts_with("bridge")
        || n.starts_with("ap")
        || n.starts_with("anpi")
        || n.starts_with("gif")
        || n.starts_with("stf")
        || n.starts_with("lo")
        || n.starts_with("pktap")
        || n.starts_with("vmnet")
        || n.starts_with("vbox")
        || n.starts_with("docker")
        || n.starts_with("br-")
        || n.starts_with("veth")
}

fn is_usable_ip(ip: &Ipv4Addr) -> bool {
    !ip.is_loopback() && !ip.is_link_local() && !ip.is_broadcast() && !ip.is_unspecified()
}

pub fn get_local_ip() -> Option<String> {
    let ifas = list_afinet_netifas().ok()?;

    let mut physical_ips: Vec<(String, String)> = Vec::new();
    let mut fallback_ips: Vec<(String, String)> = Vec::new();

    for (name, ip) in &ifas {
        if let std::net::IpAddr::V4(ipv4) = ip {
            if !is_usable_ip(ipv4) || is_virtual_interface(name) {
                continue;
            }
            let ip_str = ipv4.to_string();
            if is_physical_interface(name) {
                physical_ips.push((name.clone(), ip_str));
            } else if ipv4.is_private() {
                fallback_ips.push((name.clone(), ip_str));
            }
        }
    }

    // Best: en0 / eth0 (primary interface)
    for primary in &["en0", "eth0", "wlan0"] {
        if let Some((_, ip)) = physical_ips.iter().find(|(n, _)| n == primary) {
            return Some(ip.clone());
        }
    }

    // Then any physical interface
    if let Some((_, ip)) = physical_ips.first() {
        return Some(ip.clone());
    }

    // Fallback to non-virtual private IPs
    fallback_ips.first().map(|(_, ip)| ip.clone())
}

pub fn get_all_local_ips() -> Vec<String> {
    let ifas = match list_afinet_netifas() {
        Ok(ifas) => ifas,
        Err(_) => return vec![],
    };

    let mut ips: Vec<String> = Vec::new();
    for (name, ip) in &ifas {
        if let std::net::IpAddr::V4(ipv4) = ip {
            if !is_usable_ip(ipv4) || is_virtual_interface(name) {
                continue;
            }
            if is_physical_interface(name) || ipv4.is_private() {
                ips.push(ipv4.to_string());
            }
        }
    }
    ips.sort();
    ips.dedup();
    ips
}
