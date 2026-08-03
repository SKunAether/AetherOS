// AetherOS 2.0 桌面端应用主入口
// 初始化运行时、注册命令、创建主窗口。
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    aetheros_desktop_lib::run()
}
