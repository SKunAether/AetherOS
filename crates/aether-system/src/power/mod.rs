//! powercfg 封装：电源计划列表/激活、休眠开关
//!
//! 对齐旧版 C# `WindowsPowerPlanService` / `WindowsHibernateService`。

use aether_core::errors::CapabilityError;
use regex::Regex;
use uuid::Uuid;

/// 内置电源计划 GUID
pub const GUID_BALANCED: &str = "381b4222-f694-41f0-9685-ff5bb260df2e";
pub const GUID_HIGH_PERFORMANCE: &str = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
pub const GUID_POWER_SAVER: &str = "a1841308-3541-4fab-bc81-f71556f20b4a";

/// 电源计划信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerPlan {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub category: String,
}

/// 读取电源计划列表（`powercfg /list` + `powercfg /getactivescheme`）
pub fn list_plans() -> Result<Vec<PowerPlan>, CapabilityError> {
    let out = crate::process::run("powercfg.exe", &["/list"])?;
    let list = String::from_utf8_lossy(&out.stdout).to_string();
    let active = crate::process::run("powercfg.exe", &["/getactivescheme"])?;
    let active_line = String::from_utf8_lossy(&active.stdout).to_string();

    let active_guid = extract_first_guid(&active_line);

    let guid_re = Regex::new(
        r"\b([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})\b",
    )
    .unwrap();
    let name_re = Regex::new(r"\(([^)]+)\)\s*$").unwrap();

    let mut plans = Vec::new();
    for line in list.lines() {
        let Some(cap) = guid_re.captures(line) else {
            continue;
        };
        let id = cap[1].to_string();
        let name = name_re
            .captures(line)
            .map(|m| m[1].to_string())
            .unwrap_or_else(|| "自定义方案".to_string());
        let category = categorize(&id, &name);
        plans.push(PowerPlan {
            id: id.clone(),
            name,
            is_active: active_guid.as_ref() == Some(&id),
            category,
        });
    }

    Ok(plans)
}

/// 激活电源计划（需管理员，`powercfg /setactive <guid>`）
pub fn activate_plan(id: Uuid) -> Result<i32, CapabilityError> {
    crate::process::run_elevated("powercfg.exe", &["/setactive", &id.to_string()])
}

/// 开启/关闭休眠（需管理员，`powercfg /hibernate on|off`）
pub fn set_hibernate(enabled: bool) -> Result<i32, CapabilityError> {
    let arg = if enabled { "on" } else { "off" };
    crate::process::run_elevated("powercfg.exe", &["/hibernate", arg])
}

fn extract_first_guid(text: &str) -> Option<String> {
    let re = Regex::new(
        r"\b([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})\b",
    )
    .unwrap();
    re.captures(text).map(|c| c[1].to_string())
}

fn categorize(id: &str, name: &str) -> String {
    let lname = name.to_lowercase();
    if id.eq_ignore_ascii_case(GUID_BALANCED)
        || lname.contains("balanced")
        || lname.contains("平衡")
    {
        "Balanced".to_string()
    } else if id.eq_ignore_ascii_case(GUID_HIGH_PERFORMANCE)
        || lname.contains("high performance")
        || lname.contains("高性能")
    {
        "HighPerformance".to_string()
    } else if id.eq_ignore_ascii_case(GUID_POWER_SAVER)
        || lname.contains("power saver")
        || lname.contains("节能")
    {
        "PowerSaver".to_string()
    } else {
        "Custom".to_string()
    }
}
