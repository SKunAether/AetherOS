//! AetherOS 2.0 桌面端壳层
//!
//! 组装各 crate：初始化 aether-runtime（注册中心/事件总线/调度器），
//! 注册所有内置 Provider 与 IPC 命令，搭建事件 relay（bus → Tauri 事件），
//! 配置系统托盘、关闭到托盘、开机自启与静默启动。

pub mod ai;
pub mod commands;
pub mod error;
pub mod providers;
pub mod settings;
pub mod startup;
pub mod state;
pub mod tray;

use aether_core::models::settings::CloseBehavior;
use aether_runtime::RuntimeContext;
use state::AppState;
use tauri::{Emitter, Manager};

/// 应用初始化与启动入口
pub fn run() {
    // tracing 初始化（env-filter，默认 info）
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("aetheros=debug,info")),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 1) 构建运行时（注册中心 + 事件总线 + 调度器）
            let runtime = RuntimeContext::bootstrap();
            // 2) 系统引擎
            let system = aether_system::SystemEngine::new();
            // 3) 注册内置 Provider（demo/cleaner/monitor/optimizer/recovery）
            providers::register_builtin(&runtime.registry, &system);
            // 4) 加载设置
            let settings = settings::handle(settings::load());
            // 5) 注入共享状态
            app.manage(AppState {
                runtime,
                system,
                settings: settings.clone(),
            });
            // 6) 事件 relay：bus → app.emit("aetheros://event")
            let bus = app.state::<AppState>().runtime.bus.clone();
            spawn_event_relay(app.handle().clone(), bus);

            // 7) 系统托盘
            let _ = tray::build_tray(app.handle());

            // 8) 关闭到托盘：根据 CloseBehavior 拦截关闭事件
            let app_handle = app.handle().clone();
            if let Some(window) = app.get_webview_window("main") {
                let tray_window = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        let state = app_handle.state::<AppState>();
                        let close_behavior = state.settings.read().unwrap().close_behavior;
                        if close_behavior == CloseBehavior::MinimizeToTray {
                            api.prevent_close();
                            let _ = tray_window.hide();
                        }
                    }
                });
            }

            // 9) 开机自启同步：启动时确保注册表与设置一致
            let run_at_startup = settings.read().unwrap().run_at_startup;
            if let Ok(exe) = std::env::current_exe() {
                let _ = startup::apply(&exe.to_string_lossy(), run_at_startup);
            }

            // 10) 静默启动：--tray 参数（自启/计划任务拉起）时不显示主窗口
            let silent = std::env::args().any(|a| a == "--tray" || a == "--scheduled-scan");
            if silent {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::get_system_info,
            commands::system::open_external,
            commands::runtime::get_providers,
            commands::runtime::provider_execute,
            commands::runtime::provider_scan,
            commands::runtime::ping,
            commands::runtime::emit_test,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::quick_scan::get_quick_scan,
            commands::cleaner::cleaner_scan,
            commands::cleaner::cleaner_execute,
            commands::power::get_power_plans,
            commands::power::activate_power_plan,
            commands::hibernate::get_hibernate_state,
            commands::hibernate::set_hibernate_enabled,
            commands::ai::get_ai_providers,
            commands::ai::save_ai_provider,
            commands::ai::delete_ai_provider,
            commands::ai::test_ai_provider,
            commands::ai::run_ai_analysis,
            commands::ai::ai_configured,
            commands::ai::ai_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running aetheros-desktop");
}

/// 事件 relay：订阅运行时事件总线，转发为 Tauri 前端事件
fn spawn_event_relay(app: tauri::AppHandle, bus: std::sync::Arc<aether_runtime::bus::EventBus>) {
    let mut rx = bus.subscribe();
    tauri::async_runtime::spawn(async move {
        while let Ok(evt) = rx.recv().await {
            let json = serde_json::to_value(&evt).unwrap_or_default();
            let _ = app.emit("aetheros://event", json);
        }
    });
}
