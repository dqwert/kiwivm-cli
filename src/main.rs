use std::env;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

fn main() {
    let mut vps = KiwivmCLI {
        veid: env::var("KIWIVM_VEID").unwrap(),
        api_key: env::var("KIWIVM_API_KEY").unwrap(),
        info: None,
    };
    // vps.get_live_service_info();
    vps.print_live_service_info();
    // vps.start();
    // vps.stop();
    // vps.restart();
    vps.kill(false);
}


struct KiwivmCLI {
    veid: String,
    api_key: String,
    info: Option<VirtualPrivateServerLiveInfo>,
}

impl KiwivmCLI {
    #[tokio::main]
    async fn get_live_service_info(&mut self) -> Result<(), reqwest::Error> {
        let url = format!("https://api.64clouds.com/v1/getLiveServiceInfo?veid={}&api_key={}", self.veid, self.api_key);
        let res = reqwest::get(url).await.expect("Request failed.");
        println!("Status: {}", res.status());
        self.info = serde_json::from_str(&res.text().await?.as_str()).expect("JSON was not well-formatted.");

        // println!("{}", res.text().await?);       // for debug

        Ok(())
    }

    fn print_live_service_info(&mut self) {
        println!("{:#?}",
                 match &self.info {
                     Some(vps_info) => vps_info,
                     None => {
                         println!("KiwivmCLI.info not initialized, calling KiwivmCLI.get_service_info first.");
                         self.get_live_service_info();
                         self.info.as_ref().unwrap()
                     }
                 }
        );
    }

    #[tokio::main]
    async fn start(self) {
        let url = format!("https://api.64clouds.com/v1/start?veid={}&api_key={}", self.veid, self.api_key);
        let res = reqwest::get(url).await.expect("Request failed.");
        println!("Status: {}\nBody: {}", res.status(), res.text().await.unwrap());
    }

    #[tokio::main]
    async fn stop(self) {
        let url = format!("https://api.64clouds.com/v1/stop?veid={}&api_key={}", self.veid, self.api_key);
        let res = reqwest::get(url).await.expect("Request failed.");
        println!("Status: {}\nBody: {}", res.status(), res.text().await.unwrap());
    }

    #[tokio::main]
    async fn restart(self) {
        let url = format!("https://api.64clouds.com/v1/restart?veid={}&api_key={}", self.veid, self.api_key);
        let res = reqwest::get(url).await.expect("Request failed.");
        println!("Status: {}\nBody: {}", res.status(), res.text().await.unwrap());
    }

    #[tokio::main]
    async fn kill(self, confirm: bool) {
        if confirm {
            let url = format!("https://api.64clouds.com/v1/kill?veid={}&api_key={}", self.veid, self.api_key);
            let res = reqwest::get(url).await.expect("Request failed.");
            println!("Status: {}\nBody: {}", res.status(), res.text().await.unwrap());
        } else {
            println!("Operation `kill` not performed, please confirm supplying argument `confirm`.");
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct VirtualPrivateServerLiveInfo {
    vm_type: String,
    ve_status: String,
    ve_mac1: String,
    ve_used_disk_space_b: i64,
    ve_disk_quota_gb: String,
    is_cpu_throttled: String,
    is_disk_throttled: String,
    ssh_port: i32,
    live_hostname: String,
    load_average: String,
    mem_available_kb: i64,
    swap_total_kb: Option<i64>,
    swap_available_kb: Option<i64>,
    hostname: String,
    node_ip: String,
    node_alias: String,
    node_location: String,
    node_location_id: String,
    node_datacenter: String,
    location_ipv6_ready: bool,
    plan: String,
    plan_monthly_data: i64,
    monthly_data_multiplier: i8,
    plan_disk: i64,
    plan_ram: i64,
    plan_swap: i64,
    plan_max_ipv6s: i8,
    os: String,
    email: String,
    data_counter: i64,
    data_next_reset: i64,
    ip_addresses: Vec<String>,
    private_ip_addresses: Vec<String>,
    // ip_nullroutes: Vec<String>,      // Cannot find docs about this nor infer form existing data
    iso1: Option<String>,
    iso2: Option<String>,
    available_isos: Vec<String>,
    plan_private_network_available: bool,
    location_private_network_available: bool,
    rdns_api_available: bool,
    ptr: HashMap<String, Option<String>>,
    suspended: bool,
    policy_violation: bool,
    suspension_count: Option<i32>,
    total_abuse_points: i32,
    max_abuse_points: i32,
    error: i32,
    veid: i64,
}
