/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::fs;
use std::io::BufRead;

/// Function to read a numeric value from a file
fn read_value_from_file(path: &str) -> u64 {
    fs::read_to_string(path).unwrap_or_default().trim().parse().unwrap_or(0)
}

/// Function to read a specific value from /proc/meminfo
fn read_meminfo_value(key: &str) -> u64 {
    let meminfo_content = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    meminfo_content.lines().find(|line| line.starts_with(key)).and_then(|line| line.split_whitespace().nth(1)).unwrap_or("0").parse().unwrap_or(0)
}

/// Returns total system memory
pub fn get_total_system_memory() -> u64 {
    read_meminfo_value("MemTotal:")
}

/// Returns total swap memory
pub fn get_total_system_swap() -> u64 {
    read_meminfo_value("SwapTotal:")
}

/// Returns total used system memory
pub fn get_total_used_system_memory() -> u64 {
    let mem_total = get_total_system_memory();
    let mem_free = read_meminfo_value("MemAvailable:");
    mem_total - mem_free
}

/// Returns total used swap memory
pub fn get_total_used_system_swap() -> u64 {
    let swap_total = get_total_system_swap();
    let swap_free = read_meminfo_value("SwapFree:");
    swap_total - swap_free
}

/// Returns system total uptime
pub fn get_system_uptime() -> u64 {
    read_value_from_file("/proc/uptime")
}

/// Returns system cpu/core count
pub fn get_system_cpus() -> u64 {
    let cpu_info = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    cpu_info.lines().filter(|line| line.starts_with("processor")).count() as u64
}

/// Returns operating system version
pub fn get_operating_system() -> String {
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let version = os_release.lines().find(|line| line.starts_with("PRETTY_NAME=")).map(|line| line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string()).unwrap_or("Unknown".to_string());
    version
}