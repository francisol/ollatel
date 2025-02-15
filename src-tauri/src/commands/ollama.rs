use std::{
    collections::HashMap,
    fs,
    os::windows::process::CommandExt,
    process::{Child, Command},
    sync::Mutex,
};

use serde_json::Value;
use tauri::{path::BaseDirectory, Manager, State};
use tauri_plugin_store::StoreExt;

use super::{new_command, run_cmd, run_command};
pub struct ProcessHandle(pub Mutex<Option<Child>>);

macro_rules! insert_envs {
    ($store:expr, $envs:expr) => {
        let custom_envs = $store.get("env");
        match custom_envs {
            Some(serde_json::Value::Object(val)) => {
                for (key, value) in val.iter() {
                    if let serde_json::Value::String(v) = value {
                        if !v.is_empty() {
                            $envs.insert(key.to_string(), v.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    };
}

#[tauri::command]
pub fn install_ollama<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    // init-ollama.bat
    let ollama_dir = app.path().app_data_dir().unwrap().join("ollama");
    let script = app
        .path()
        .resolve("resources/install_ollama.py", BaseDirectory::Resource)
        .unwrap();
    let io_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("python")
        .join("install_ollama.py");
    fs::copy(script, &io_path).map_err(|e| e.to_string())?;
    let dir = (&ollama_dir).to_str().unwrap_or_default();
    let _ = fs::create_dir_all(&ollama_dir);
    let argument_list = format!("\"{} {}\"", io_path.to_str().unwrap(), dir);
    run_command(
        &app,
        "powershell",
        vec![
            "Start-Process",
            "python.exe",
            "-Verb",
            "RunAs",
            "-ArgumentList",
            &argument_list,
        ],
        &ollama_dir,
        "install-ollama",
        HashMap::new(),
    )
    // run_command(&app,"runas",&vec!["/user:Administrator",
    // "init-ollama.bat",],ollama_dir,"install-ollama",HashMap::new())
}

fn is_runing(p: &mut Child) -> Result<bool, String> {
    match p.try_wait() {
        Ok(Some(_)) => Ok(false),
        Ok(None) => Ok(true),
        Err(err) => Err(err.to_string()),
    }
}
#[tauri::command]
pub fn is_ollama_running<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    process_handle: State<ProcessHandle>,
) -> Result<bool, String> {
    let mut handle = process_handle.0.lock().unwrap();

    // 如果进程已经在运行，返回错误
    match handle.as_mut() {
        Some(p) => {
            let r = is_runing(p)?;
            if !r {
                *handle = None;
            }
            Ok(r)
        }
        None => Ok(false),
    }
}

#[tauri::command]
pub fn stop_ollama<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    process_handle: State<ProcessHandle>,
) -> Result<(), String> {
    let mut handle = process_handle.0.lock().unwrap();

    match handle.as_mut() {
        Some(p) => {
            kill_process_tree(p.id());
            *handle = None;
            Ok(())
        }
        None => Err("Process is not running".to_string()),
    }
}

#[tauri::command]
pub fn run_ollama<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    process_handle: State<ProcessHandle>,
) -> Result<(), String> {
    let mut handle = process_handle.0.lock().unwrap();
    // 如果进程已经在运行，返回错误

    match handle.as_mut() {
        Some(p) => {
            let running = is_runing(p)?;
            if running {
                return Err("Process is already running".to_string());
            }
        }
        None => {}
    }
    let ollama_dir = app.path().app_data_dir().unwrap().join("ollama");

    let mut envs = HashMap::new();
    envs.insert("OLLAMA_NUM_GPU".into(), "9999".into());
    envs.insert("no_proxy".into(), "localhost,127.0.0.1".into());
    envs.insert("ZES_ENABLE_SYSMAN".into(), "1".into());
    envs.insert("OLLAMA_FLASH_ATTENTION".into(), "True".into());
    envs.insert(
        "SYCL_PI_LEVEL_ZERO_USE_IMMEDIATE_COMMANDLISTS".into(),
        "1".into(),
    );
    envs.insert("GIN_MODE".into(), "release".into());
    let store = app.store("env.bin").map_err(|e| e.to_string())?;
    let custom_store = app.store("custom_data.bin").map_err(|e| e.to_string())?;
    insert_envs!(store,envs);
    insert_envs!(custom_store,envs);
    let cmd = run_cmd(&app, "ollama", ["serve"], ollama_dir, "run-ollama", envs)?;
    *handle = Some(cmd);
    Ok(())
}

#[tauri::command]
pub fn open_terminal<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    terminal_type: &str,
) -> Result<(), String> {
    let ollama_dir = app.path().app_data_dir().unwrap().join("ollama");
    let mut args = vec!["/C", "start"];
    match terminal_type {
        "cmd" => {
            args.extend(["cmd", "/k", "title", "Ollama 环境"]);
        }
        "powershell" => {
            args.extend([
                "powershell",
                "-NoExit",
                "-Command",
                "$Host.UI.RawUI.WindowTitle = 'Ollama 环境'",
            ]);
        }
        _ => return Err(format!("Invalid terminal type {}", &terminal_type)),
    }
    new_command(&app, "cmd", args, ollama_dir, HashMap::new())?
        .creation_flags(super::CREATE_NO_WINDOW | super::DETACHED_PROCESS) // DETACHED_PROCESS
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn kill_process_tree(pid: u32) {
    let _ = Command::new("taskkill")
        .args(["/F", "/T", "/PID", &pid.to_string()])
        .creation_flags(super::CREATE_NO_WINDOW)
        .output();
}

pub fn on_exit<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    let process = app.state::<ProcessHandle>();
    let mut handle = process.0.lock().unwrap();

    match handle.as_mut() {
        Some(p) => {
            kill_process_tree(p.id());
        }
        None => {}
    }
}
