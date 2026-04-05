use local_ip_address::list_afinet_netifas;

pub fn get_local_ip() -> Option<String> {
    let ifas = list_afinet_netifas().ok()?;

    let mut candidates: Vec<(String, String)> = Vec::new();

    for (name, ip) in &ifas {
        if let std::net::IpAddr::V4(ipv4) = ip {
            let ip_str = ipv4.to_string();
            if ip_str == "127.0.0.1" {
                continue;
            }
            if ipv4.is_private() {
                candidates.push((name.clone(), ip_str));
            }
        }
    }

    // Prefer en0 (WiFi on macOS)
    if let Some((_, ip)) = candidates.iter().find(|(name, _)| name == "en0") {
        return Some(ip.clone());
    }

    // Then prefer 192.168.x.x
    if let Some((_, ip)) = candidates
        .iter()
        .find(|(_, ip)| ip.starts_with("192.168."))
    {
        return Some(ip.clone());
    }

    // Then 10.x.x.x
    if let Some((_, ip)) = candidates.iter().find(|(_, ip)| ip.starts_with("10.")) {
        return Some(ip.clone());
    }

    candidates.first().map(|(_, ip)| ip.clone())
}

pub fn get_all_local_ips() -> Vec<String> {
    let ifas = match list_afinet_netifas() {
        Ok(ifas) => ifas,
        Err(_) => return vec![],
    };

    let mut ips: Vec<String> = Vec::new();
    for (_name, ip) in &ifas {
        if let std::net::IpAddr::V4(ipv4) = ip {
            let ip_str = ipv4.to_string();
            if ip_str != "127.0.0.1" && ipv4.is_private() {
                ips.push(ip_str);
            }
        }
    }
    ips.sort();
    ips.dedup();
    ips
}
