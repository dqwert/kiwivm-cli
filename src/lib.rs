use std::collections::HashMap;
use std::env;
use std::process;

use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Error;


pub struct KiwivmCLI {
    pub veid: String,
    pub api_key: String,
    pub info: Option<VirtualPrivateServerLiveInfo>,
}


impl KiwivmCLI {
    pub fn new() -> KiwivmCLI {
        KiwivmCLI {
            veid: env::var("KIWIVM_VEID").unwrap(),
            api_key: env::var("KIWIVM_API_KEY").unwrap(),
            info: None,
        }
    }

    pub fn call_api(&self, operation: Operation) -> Result<(), reqwest::Error> {
        let url = self.construct_url(operation);
        let response = reqwest::blocking::get(url)?;
        let body = response.text()?;
        // let json_data: serde_json::Value = serde_json::from_str(body.as_str()).expect("Parsing body error");

        println!("Result:\n{}", body);

        Ok(())
    }

    #[allow(non_snake_case)]
    fn construct_url(&self, operation: Operation) -> String {
        let call = match operation {
            Operation::Start => "start",
            Operation::Stop => "stop",
            Operation::Restart => "restart",
            Operation::Kill => "kill",
            Operation::GetLiveServiceInfo => "getLiveServiceInfo",
            Operation::GetAvailableOS => "getAvailableOS",
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
        };
        let mut url = format!("https://api.64clouds.com/v1/{}?veid={}&api_key={}", &call, self.veid, self.api_key);
        url.push_str(match operation {
            Operation::ReinstallOs { os } =>
                format!("&os={}", os),

            Operation::SetHostname { newHostname } =>
                format!("&newHostname={}", newHostname),

            Operation::SetPtr { ip, ptr } =>
                format!("&ip={}&ptr={}", ip, ptr),

            Operation::BasicShellCd { currentDir, newDir } =>
                format!("&currentDir={}&newDir={}", currentDir, newDir),

            Operation::BasicShellExec { command } =>
                format!("&command={}", command),

            Operation::ShellScriptExec { script } =>
                format!("&script={}", script),

            Operation::SnapshotCreate { description } =>
                match description {
                    None => "".to_string(),
                    Some(description) => format!("&description={}", description),
                },

            Operation::SnapshotDelete { snapshot } =>
                format!("&snapshot={}", snapshot),

            Operation::SnapshotRestore { snapshot } =>
                format!("&snapshot={}", snapshot),

            Operation::SnapshotToggleSticky { snapshot, sticky } =>
                format!("&snapshot={}&sticky={}", snapshot, if sticky { "1" } else { "0" }),

            Operation::SnapshotExport { snapshot } =>
                format!("&snapshot={}", snapshot),

            Operation::SnapshotImport { sourceVeid, sourceToken } =>
                format!("&sourceVeid={}&sourceToken={}", sourceVeid, sourceToken),

            Operation::Ipv6Add { ip } =>
                match ip {
                    None => "".to_string(),
                    Some(ip) => format!("&ip={}", ip).to_string(),
                },

            Operation::Ipv6Delete { ip } =>
                format!("&ip={}", ip),

            Operation::MigrateStart { location } =>
                format!("&location={}", location),

            Operation::CloneFromExternalServer {
                externalServerIP,
                externalServerSSHport,
                externalServerRootPassword
            } =>
                format!("&externalServerIP={}&externalServerSSHport={}&externalServerRootPassword={}",
                        externalServerIP, externalServerSSHport, externalServerRootPassword),

            Operation::Unsuspend { record_id } =>
                format!("&record_id={}", record_id),

            _ => "".to_string(),
        }.as_str());
        url
    }

    pub fn hint() -> &'static str {
"The KiWiVM CLI tool

USAGE:
    kiwivmcil <OPTIONS>

OPTIONS:
    start               Starts the VPS
    stop                Stops the VPS
    restart             Reboots the VPS
    kill                Allows to forcibly stop a VPS that is stuck and cannot be stopped by normal means.
    getLiveServiceInfo  This function returns all data provided by getServiceInfo. In addition, it provides detailed status of the VPS.
    getAvailableOS      Get available OS
    reinstallOS <os>    Reinstall the Operating System. OS must be specified via \"os\" variable.
    resetRootPassword   Generates and sets a new root password.
    getUsageGraphs      Obsolete, use getRawUsageStats instead
    getRawUsageStats    Returns a two-dimensional array with the detailed usage statistics shown under Detailed Statistics in KiwiVM.

    setHostname <newHostname>       Sets new hostname.
    setPTR <ip> <ptr>               Sets new PTR (rDNS) record for IP.
    basicShell/cd <currentDir> <newDir>
                                    Simulate change of directory inside of the VPS.
    basicShell/exec <command>       Execute a shell command on the VPS (synchronously).
    shellScript/exec <script>       Execute a shell script on the VPS (asynchronously).
    snapshot/create [description]   Create snapshot
    snapshot/list                   Get list of snapshots.
    snapshot/delete <snapshot>      Delete snapshot by fileName (can be retrieved with snapshot/list call).
    snapshot/restore <snapshot>     Restores snapshot by fileName (can be retrieved with snapshot/list call). This will overwrite all data on the VPS.
    snapshot/toggleSticky <snapshot> <sticky>
                                    Set or remove sticky attribute. Name of snapshot can be retrieved with snapshot/list call – look for fileName variable.
    snapshot/export <snapshot>      Generates a token with which the snapshot can be transferred to another instance.
    snapshot/import <sourceVeid> <sourceToken>
                                    Imports a snapshot from another instance identified by VEID and Token. Both VEID and Token must be obtained from another instance beforehand with a snapshot/export call.
    ipv6/add [ip]                   Assigns a new IPv6 address. For initial IPv6 assignment an empty IP is required (call without parameters), and a new IP from the available pool is assigned automatically. All subsequent requested IPv6 addresses must be within the /64 subnet of the first IPv6 address.
    ipv6/delete <ip>                Releases specified IPv6 address.
    migrate/getLocations            Return all possible migration locations.
    migrate/start <location>        Start VPS migration to new location. Takes new location ID as input.
    cloneFromExternal <ServerexternalServerIP> <externalServerSSHport> <externalServerRootPassword>
                                    (OVZ only) Clone a remote server or VPS. See Migrate from another server for example on how this works.
    getSuspensionDetails            Retrieve information related to service suspensions.
    unsuspend <record_id>           Clear abuse issue identified by record_id and unsuspend the VPS. Refer to getSuspensionDetails call for details.
    getRateLimitStatus              Monitoring API limit."
    }
}


#[allow(non_snake_case)]
pub enum Operation {
    Start,
    Stop,
    Restart,
    Kill,
    GetLiveServiceInfo,
    GetAvailableOS,
    ReinstallOs { os: String },
    ResetRootPassword,
    GetUsageGraphs,
    GetRawUsageStats,
    SetHostname { newHostname: String },
    SetPtr { ip: String, ptr: String },
    BasicShellCd { currentDir: String, newDir: String },
    BasicShellExec { command: String },
    ShellScriptExec { script: String },
    SnapshotCreate { description: Option<String> },
    SnapshotList,
    SnapshotDelete { snapshot: String },
    SnapshotRestore { snapshot: String },
    SnapshotToggleSticky { snapshot: String, sticky: bool },
    SnapshotExport { snapshot: String },
    SnapshotImport { sourceVeid: String, sourceToken: String },
    Ipv6Add { ip: Option<String> },
    Ipv6Delete { ip: String },
    MigrateGetLocations,
    MigrateStart { location: String },
    CloneFromExternalServer {
        externalServerIP: String,
        externalServerSSHport: String,
        externalServerRootPassword: String,
    },
    GetSuspensionDetails,
    Unsuspend { record_id: String },
    GetRateLimitStatus,
}


impl Operation {
    pub fn new(args: Vec<String>) -> Result<Operation, Error> {
        if args.len() < 2 {
            eprint!("No Operation given.");

            process::exit(-1);
        }

        match args[1].as_str() {
            "start" => Ok(Operation::Start),
            "stop" => Ok(Operation::Stop),
            "restart" => Ok(Operation::Restart),
            "kill" => Ok(Operation::Kill),
            "getServiceInfo" => {
                eprint!("Use `getLiveServiceInfo` instead. ");
                process::exit(-1);
            },
            "getLiveServiceInfo" => Ok(Operation::GetLiveServiceInfo),
            "getAvailableOS" => Ok(Operation::GetAvailableOS),
            "reinstallOS" => {
                if args.len() < 2 { eprint!("Too few arguments") }

                Ok(Operation::ReinstallOs { os: args[2].clone() })
            },
            "resetRootPassword" => Ok(Operation::ResetRootPassword),
            "getUsageGraphs" => Ok(Operation::GetUsageGraphs),
            "getRawUsageStats" => Ok(Operation::GetRawUsageStats),
            "setHostname" => {
                check_args_len(&args, 3);

                Ok(Operation::SetHostname { newHostname: args[2].clone() })
            },
            "setPTR" => {
                check_args_len(&args, 4);

                Ok(Operation::SetPtr { ip: args[2].clone(), ptr: args[3].clone() })
            },
            "basicShell/cd" => {
                check_args_len(&args, 4);

                Ok(Operation::BasicShellCd { currentDir: args[2].clone(), newDir: args[3].clone() })
            },
            "basicShell/exec" => {
                check_args_len(&args, 3);

                Ok(Operation::BasicShellExec { command: args[2].clone() })
            },
            "shellScript/exec" => {
                check_args_len(&args, 3);

                Ok(Operation::ShellScriptExec { script: args[2].clone() })
            },
            "snapshot/create" => {
                Ok(Operation::SnapshotCreate { description: None })
            },
            "snapshot/list" => Ok(Operation::SnapshotList),
            "snapshot/delete" => {
                check_args_len(&args, 3);

                Ok(Operation::SnapshotDelete { snapshot: args[2].clone() })
            },
            "snapshot/restore" => {
                check_args_len(&args, 3);

                Ok(Operation::SnapshotRestore { snapshot: args[2].clone() })
            },
            "snapshot/toggleSticky" => {
                check_args_len(&args, 3);
                let is_sticky =
                    if args[3].contains("true") || args[3].contains("1") { true }
                    else if args[3].contains("false") || args[3].contains("0")  { false }
                    else {
                        eprint!("invalid option for sticky");
                        process::exit(-1);
                    };
                Ok(Operation::SnapshotToggleSticky { snapshot: args[2].clone(), sticky: is_sticky })
            },
            "snapshot/export" => {
                check_args_len(&args, 3);

                Ok(Operation::SnapshotExport { snapshot: args[2].clone() })
            },
            "snapshot/import" => {
                check_args_len(&args, 3);

                Ok(Operation::SnapshotImport { sourceVeid: args[2].clone(), sourceToken: args[3].clone() })
            },
            "ipv6/add" => {
                if args.len() < 2 { Ok(Operation::Ipv6Add { ip: None }) }
                else { Ok(Operation::Ipv6Add { ip: None }) }
            },
            "ipv6/delete" => {
                check_args_len(&args, 3);

                Ok(Operation::Ipv6Delete { ip: args[2].clone() })
            },
            "migrate/getLocations" => Ok(Operation::MigrateGetLocations),
            "migrate/start" => {
                check_args_len(&args, 3);

                Ok(Operation::MigrateStart { location: args[2].clone() })
            },
            "cloneFromExternalServer" => {
                check_args_len(&args, 5);

                Ok(Operation::CloneFromExternalServer {
                    externalServerIP: args[2].clone(),
                    externalServerSSHport: args[3].clone(),
                    externalServerRootPassword: args[4].clone(),
                })
            },
            "getSuspensionDetails" => Ok(Operation::GetSuspensionDetails),
            "unsuspend" => {
                check_args_len(&args, 3);

                Ok(Operation::Unsuspend { record_id: args[2].clone() })
            },
            "getRateLimitStatus" => Ok(Operation::GetRateLimitStatus),
            _ => {
                eprint!("Invalid operation.");
                process::exit(-1);
            },
        }
    }
}

fn check_args_len(args: &Vec<String>, len: usize) {
    if args.len() < len {
        eprint!("Too few arguments, need {}, you supplied {}", len, args.len());

        process::exit(-1);
    }
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
