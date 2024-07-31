use anyhow::{anyhow, Result};
use regex::Regex;
#[allow(unused)]
#[derive(Debug)]
struct NginxLog {
    addr: String,
    date: String,
    method: String,
    url: String,
    protocol: String,
    status: u16,
    body_bytes: u64,
    referer: String,
    user_agent: String,
}

fn main() -> Result<()> {
    let s = r#"93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)""#;
    let log = parse_nginx_log(s)?;
    println!("{:?}", log);
    Ok(())
}

fn parse_nginx_log(s: &str) -> Result<NginxLog> {
    let re = Regex::new(
        r#"^(?<ip>\S+)\s+\S+\s+\S+\s+\[(?<date>[^\]]+)\]\s+"(?<method>\S+)\s+(?<url>\S+)\s+(?<proto>[^"]+)"\s+(?<status>\d+)\s+(?<bytes>\d+)\s+"(?<referer>[^"]+)"\s+"(?<ua>[^"]+)"$"#,
    )?;
    let cap = re.captures(s).ok_or(anyhow!("invalid log format"))?;
    Ok(NginxLog {
        addr: cap["ip"].to_string(),
        date: cap["date"].to_string(),
        method: cap["method"].to_string(),
        url: cap["url"].to_string(),
        protocol: cap["proto"].to_string(),
        status: cap["status"].parse()?,
        body_bytes: cap["bytes"].parse()?,
        referer: cap["referer"].to_string(),
        user_agent: cap["ua"].to_string(),
    })
}
