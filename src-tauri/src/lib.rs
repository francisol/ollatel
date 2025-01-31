use std::{fs, sync::Mutex};

use commands::{ollama::ProcessHandle, register_commands};
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    Manager,
};
use tauri_plugin_autostart::MacosLauncher;

pub mod commands;

// 创建系统托盘菜单
fn create_tray_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
    let menus = Menu::with_items(app, &[&show, &quit]).unwrap();
    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menus)
        .show_menu_on_left_click(true)
        .on_menu_event(handle_tray_event)
        .on_tray_icon_event(|_t, e| {})
        .build(app);
}

// 处理托盘事件
fn handle_tray_event<R: tauri::Runtime>(app: &tauri::AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        "quit" => {
            commands::ollama::on_exit(app);
            app.exit(0);
        }
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    register_commands(
        tauri::Builder::default()
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
                let _ = app
                    .get_webview_window("main")
                    .expect("no main window")
                    .set_focus();
            }))
            // .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                None,
            ))
            .plugin(tauri_plugin_store::Builder::new().build())
            .manage(ProcessHandle(Mutex::new(None)))
            // .plugin(tauri_plugin_os::init())
            .setup(|app| {
                fs::create_dir_all(app.path().app_data_dir()?)?;
                create_tray_menu(app.handle());
                let window = app.get_webview_window("main").unwrap();
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        w.hide().unwrap();
                    }
                });
                Ok(())
            })
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_opener::init()),
    )
    // .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
