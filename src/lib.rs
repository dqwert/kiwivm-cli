use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Map;


#[derive(Debug)]
pub struct KiwivmCLI {
    pub veid: String,
    pub api_key: String,
    pub info: Option<VirtualPrivateServerLiveInfo>,
}

impl KiwivmCLI {
    pub fn call_api(&self, operation: Operation) -> Result<(), reqwest::Error> {
        let call = KiwivmCLI::get_call(operation);
        let url = format!("https://api.64clouds.com/v1/{}?veid={}&api_key={}", &call, self.veid, self.api_key);
        let response = reqwest::blocking::get(url)?;
        let body = response.text()?;
        let json_data: serde_json::Value = serde_json::from_str(body.as_str()).expect("Parsing body error");

        println!("Calling {}, Result:\n{:?}", call, json_data);

        Ok(())
    }

    fn get_call(operation: Operation) -> &'static str {
        match operation {
            Operation::Start => "start",
            Operation::Stop => "stop",
            Operation::Restart => "restart",
            Operation::Kill => "kill",
            Operation::GetLiveServiceInfo => "getLiveServiceInfo",
            Operation::GetAvailableOs => "getAvailableOS",
            Operation::ReinstallOs { .. } => "reinstallOS",
            Operation::ResetRootPassword => "resetRootPassword",
            Operation::GetUsageGraphs => "getUsageGraphs",
            Operation::GetRawUsageStats => "getRawUsageStats",
            Operation::SetHostname { .. } => "setHostname",
            Operation::SetPtr { .. } => "setPTR",
            Operation::BasicShellCd { .. } => "basicShell/cd",
            Operation::BasicShellExec { .. } => "basicShell/exec",
            Operation::ShellScriptExec { .. } => "shellScript/exec",
            Operation::SnapshotCreate { .. } => "snapshot/create",
            Operation::SnapshotList => "snapshot/list",
            Operation::SnapshotDelete { .. } => "snapshot/delete",
            Operation::SnapshotRestore { .. } => "snapshot/restore",
            Operation::SnapshotToggleSticky { .. } => "snapshot/toggleSticky",
            Operation::SnapshotExport { .. } => "snapshot/export",
            Operation::SnapshotImport { .. } => "snapshot/import",
            Operation::Ipv6Add { .. } => "ipv6/add",
            Operation::Ipv6Delete { .. } => "ipv6/delete",
            Operation::MigrateGetLocations => "migrate/getLocations",
            Operation::MigrateStart { .. } => "migrate/start",
            Operation::CloneFromExternalServer { .. } => "cloneFromExternalServer",
            Operation::GetSuspensionDetails => "getSuspensionDetails",
            Operation::Unsuspend { .. } => "unsuspend",
            Operation::GetRateLimitStatus => "getRateLimitStatus",
        }
    }
}

pub enum Operation {
    Start,
    Stop,
    Restart,
    Kill,
    GetLiveServiceInfo,
    GetAvailableOs,
    ReinstallOs { os: String },
    ResetRootPassword,
    GetUsageGraphs,
    GetRawUsageStats,
    SetHostname { newHostname: String },
    SetPtr { ip: String, ptr: String},
    BasicShellCd { currentDir: String, newDir: String },
    BasicShellExec { command: String },
    ShellScriptExec { script: String },
    SnapshotCreate { description: Option<String>},
    SnapshotList,
    SnapshotDelete { snapshot: String },
    SnapshotRestore { snapshot: String },
    SnapshotToggleSticky { snapshot: String, sticky: bool },
    SnapshotExport { snapshot: String },
    SnapshotImport { sourceVeid: String, sourceToken: String},
    Ipv6Add { ip: Option<String> },
    Ipv6Delete { ip: String },
    MigrateGetLocations,
    MigrateStart { location: String },
    CloneFromExternalServer { externalServerIP: String, externalServerSSHport: String, externalServerRootPassword: String},
    GetSuspensionDetails,
    Unsuspend { record_id: String },
    GetRateLimitStatus,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualPrivateServerLiveInfo {
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
