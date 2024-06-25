use reqwest::redirect::Policy;
use reqwest::StatusCode;
use std::net::IpAddr;
use std::str::FromStr;
#[macro_use]
extern crate anyhow;
const IPS: &'static [&str] = &[
    "ip.sb",
    "ping0.cc",
    "icanhazip.com",
    "api64.ipify.org",
    "ifconfig.co",
    "ident.me",
];
#[tokio::main]
async fn main() {
    if let Err(e) = main_with_error().await {
        println!("{}", e);
    }
}
async fn main_with_error() -> Result<(), anyhow::Error> {
    // std::env::set_var("RUST_LOG", "trace");

    env_logger::init();
    if let Ok(ip) = get_ip().await {
        println!("{}", ip);
        db_maxmind(ip).await?;
    }
    Ok(())
}
async fn get_ip() -> Result<String, anyhow::Error> {
    let local_addr = IpAddr::from([0, 0, 0, 0]);

    let client = reqwest::Client::builder()
        .no_proxy()
        .connection_verbose(true)
        .local_address(local_addr)
        .build()?;

    for item in IPS {
        let url = format!("http://{}", item);

        let resp = client
            .get(url)
            .header("Host", item.to_string())
            .header("User-Agent", "curl/8.4.0")
            .send()
            .await?;

        if resp.status() == StatusCode::OK {
            let text: String = resp.text().await?;
            if !text.contains("error") {
                return Ok(text);
            }
        } else {
            let text: String = resp.text().await?;
            println!("{}", text);
        }
    }
    Err(anyhow!("Can not get the ip!"))
}
async fn db_maxmind(ip: String) -> Result<(), anyhow::Error> {
    let local_addr = IpAddr::from([0, 0, 0, 0]);

    let client = reqwest::Client::builder()
        .no_proxy()
        .connection_verbose(true)
        .local_address(local_addr)
        .build()?;
    let url = format!("https://ipinfo.check.place/{}?lang=zh", ip);
    let resp = client
        .get(url)
        .header("Host", "ipinfo.check.place")
        .header("User-Agent", "curl/8.4.0")
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        let text: String = resp.text().await?;
        println!("{}", text);
    }
    Ok(())
}
