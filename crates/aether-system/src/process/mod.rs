//! 进程管理：运行命令、提权执行、管理员检测
//!
//! 对齐旧版 C# CommandRunner 与 UAC 提权（Verb=runas）行为。

use aether_core::errors::CapabilityError;
use std::process::Command;

/// UAC 用户取消的错误码
pub const UAC_CANCELLED: i32 = 1223;

/// 命令输出
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// 非提权运行命令，捕获输出
pub fn run(cmd: &str, args: &[&str]) -> Result<CommandOutput, CapabilityError> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| CapabilityError::Platform(format!("failed to run {cmd}: {e}")))?;
    Ok(CommandOutput {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

/// 提权运行命令（触发 UAC，等待完成并返回子进程退出码）。
///
/// 通过 PowerShell `Start-Process -Verb RunAs -Wait -PassThru` 实现，
/// 对齐旧版 C# `ProcessStartInfo.Verb = "runas"`。用户取消 UAC 返回 1223。
pub fn run_elevated(program: &str, args: &[&str]) -> Result<i32, CapabilityError> {
    let quoted_args: Vec<String> = args
        .iter()
        .map(|a| format!("'{}'", a.replace('\'', "''")))
        .collect();
    let script = format!(
        "Start-Process -FilePath '{}' -ArgumentList @({}) -Verb RunAs -Wait -PassThru | Select-Object -ExpandProperty ExitCode",
        program.replace('\'', "''"),
        quoted_args.join(",")
    );

    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| CapabilityError::Platform(format!("elevation failed: {e}")))?;

    // PowerShell 将子进程退出码打印到 stdout；UAC 取消时无输出
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = stdout.trim().parse::<i32>().ok();
    match code {
        Some(code) if code == UAC_CANCELLED => {
            Err(CapabilityError::Unauthorized("UAC 授权被取消".to_string()))
        }
        Some(code) => Ok(code),
        None => Err(CapabilityError::Unauthorized(
            "UAC 授权被取消或无输出".to_string(),
        )),
    }
}
