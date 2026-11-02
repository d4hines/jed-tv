use super::wifi_codec::{configuration, parse_scan, Network};

fn reference_scan(text: &str) -> Vec<Network> {
    let mut networks: Vec<Network> = Vec::new();
    for line in text.lines().skip(1) {
        let fields: Vec<_> = line.splitn(5, '\t').collect();
        if fields.len() != 5 || fields[4].is_empty() || fields[4].len() > 32
            || networks.iter().any(|n| n.ssid == fields[4]) { continue; }
        let secure = fields[3].contains("PSK");
        if fields[3].contains("WEP") || (!secure && (fields[3].contains("WPA")
            || fields[3].contains("EAP") || fields[3].contains("SAE")))
            || fields[4].contains('\\') { continue; }
        networks.push(Network { ssid: fields[4].into(), signal: fields[2].into(), secure });
        if networks.len() == 24 { break; }
    }
    networks
}
fn reference_config(ssid: &str, password: &str, secure: bool) -> Result<String, String> {
    if ssid.len() > 32 || ssid.is_empty() { return Err("Invalid Wi-Fi network name".into()); }
    if secure && (!(8..=63).contains(&password.len()) || !password.bytes().all(|b| (32..=126).contains(&b))) {
        return Err("WPA password must be 8 to 63 printable ASCII characters".into());
    }
    let hex: String = ssid.bytes().map(|b| format!("{b:02x}")).collect();
    let escaped = password.replace('\\', "\\\\").replace('"', "\\\"");
    let credentials = if secure { format!("key_mgmt=WPA-PSK\npsk=\"{escaped}\"\n") }
                      else { "key_mgmt=NONE\n".into() };
    Ok(format!("ctrl_interface=/run/tv-network/wpa\nnetwork={{\nssid={hex}\n{credentials}}}\n"))
}
fn scan(text: &str) {
    assert_eq!(parse_scan(text).unwrap(), reference_scan(text));
}
fn config(ssid: &str, synthetic: &str, secure: bool) {
    assert_eq!(configuration(ssid, synthetic, secure).map_err(|e| e.to_string()),
               reference_config(ssid, synthetic, secure));
}
fn next(seed: &mut u32) -> usize {
    *seed ^= *seed << 13; *seed ^= *seed >> 17; *seed ^= *seed << 5; *seed as usize
}
pub fn check() {
    let flags = ["", "[ESS]", "[WPA2-PSK-CCMP][ESS]", "[WEP]", "[WPA2-EAP]", "[SAE]", "[WPA2-PSK-SAE]", "WEP-PSK", "PSK", "EAP-PSK"];
    let names = ["", "synthetic", "quoted\"name", "semi;colon", "slash\\name", "line\nname", "tab\tname", "nul\0name", "é", "東京", "\r", " "];
    for ending in ["", "\n", "\r", "\r\n"] {
        for flag in flags { for name in names {
            scan(&format!("heading{ending}a\t2412\t-55\t{flag}\t{name}{ending}"));
        }}
    }
    for name in names { for secure in [false, true] {
        config(name, "synthetic", secure);
        config(name, "escaped\\\"synthetic", secure);
        config(name, "", secure);
    }}
    for length in 0..70 {
        config(&"S".repeat(length), "synthetic", true);
        config("synthetic", &"p".repeat(length), true);
        config("synthetic", &"é".repeat(length), true);
    }
    for byte in 0..=127u8 {
        config("synthetic", &format!("synthetic{}", byte as char), true);
    }
    let mut seed = 0x7aba2a91;
    let chars = ['a','b','\t','\n','\r','\\','"','\0','é','東'];
    for _ in 0..2000 {
        let mut text = String::from("header\n");
        for _ in 0..(next(&mut seed) % 45) {
            let mut name = String::new();
            for _ in 0..next(&mut seed) % 36 { name.push(chars[next(&mut seed) % chars.len()]); }
            let flag = flags[next(&mut seed) % flags.len()];
            let strength = -((next(&mut seed) % 100) as i32);
            text.push_str(&format!("a\t2412\t{strength}\t{flag}\t{name}\n"));
        }
        scan(&text);
    }
    let mut capped = String::from("header\n");
    for i in 0..50 { capped.push_str(&format!("a\t2412\t-10\t[ESS]\tnetwork-{i}\n")); }
    assert_eq!(parse_scan(&capped).unwrap().len(),24);
    scan(&capped);
    println!("PASS: Lean Wi-Fi scan/filter/cap and config match native oracle, including hostile SSID lines and synthetic passphrase boundaries");
}
