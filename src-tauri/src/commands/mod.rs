use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    process::{Child, Command},
};
// use tauri_plugin_shell::ShellExt;
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use tauri::{Builder, Emitter, Manager, Runtime};

pub mod ollama;
pub mod python;

pub const DETACHED_PROCESS: u32 = 0x00000008;
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
pub async fn init_env<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().unwrap();
    fs::remove_dir_all(&data_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn register_commands<R: Runtime>(app: Builder<R>) -> Builder<R> {
    app.invoke_handler(tauri::generate_handler![
        python::install_python,
        python::install_pip,
        python::install_deps,
        ollama::install_ollama,
        ollama::run_ollama,
        ollama::open_terminal,
        ollama::is_ollama_running,
        ollama::stop_ollama,
        init_env,
    ])
}

pub fn run_command<
    R: tauri::Runtime,
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
>(
    app: &tauri::AppHandle<R>,
    program: S,
    args: I,
    current_dir: P,
    event: &str,
    mut envs: HashMap<String, String>,
) -> Result<(), String> {
    // 获取当前 PATH
    let current_path = env::var("PATH").unwrap_or_default();
    let python_dir = app.path().app_data_dir().unwrap().join("python");
    let bin_dir = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("python")
        .join("Library")
        .join("bin");
    let ollama_dir = app.path().app_data_dir().unwrap().join("ollama");
    let scripts_dir = (&python_dir).join("Scripts");
    // let extend_dir = (&python_dir).join("Lib").join("site-packages").join("bigdl").join("cpp").join("libs");
    // 添加新路径
    let new_path = format!(
        "{};{};{};{};{}",
        python_dir.to_str().unwrap_or_default(),
        scripts_dir.to_str().unwrap_or_default(),
        ollama_dir.to_str().unwrap_or_default(),
        bin_dir.to_str().unwrap_or_default(),
        // extend_dir.to_str().unwrap_or_default(),
        current_path
    );
    envs.insert("PATH".into(), new_path);

    let mut cmd = run_cmd(app, program, args, current_dir, event, envs)?;
    let _ = cmd.wait().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn new_command<
    R: tauri::Runtime,
    P: AsRef<Path>,
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
>(
    app: &tauri::AppHandle<R>,
    program: S,
    args: I,
    current_dir: P,
    mut envs: HashMap<String, String>,
) -> Result<Command, String> {
    // 获取当前 PATH
    let current_path = env::var("PATH").unwrap_or_default();
    let python_dir = app.path().app_data_dir().unwrap().join("python");
    let bin_dir = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("python")
        .join("Library")
        .join("bin");
    let ollama_dir = app.path().app_data_dir().unwrap().join("ollama");
    let scripts_dir = (&python_dir).join("Scripts");
    // let extend_dir = (&python_dir).join("Lib").join("site-packages").join("bigdl").join("cpp").join("libs");
    // 添加新路径
    let new_path = format!(
        "{};{};{};{};{}",
        python_dir.to_str().unwrap_or_default(),
        scripts_dir.to_str().unwrap_or_default(),
        ollama_dir.to_str().unwrap_or_default(),
        bin_dir.to_str().unwrap_or_default(),
        // extend_dir.to_str().unwrap_or_default(),
        current_path
    );
    envs.insert("PATH".into(), new_path);
    // app.app_handle().shell();
    let mut cmd = std::process::Command::new(program);
    cmd.current_dir(current_dir).args(args).envs(envs);
    return Ok(cmd);
}
pub fn run_cmd<R: tauri::Runtime, P: AsRef<Path>, S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
    app: &tauri::AppHandle<R>,
    program: S,
    args: I,
    current_dir: P,
    event: &str,
    envs: HashMap<String, String>,
) -> Result<Child, String> {
    let mut cmd = new_command(app, program, args, current_dir, envs)?
        .stdout(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let stdout = cmd.stdout.take().unwrap();
    let stderr = cmd.stderr.take().unwrap();
    let app_clone = app.clone();
    let output_event_name = format!("{}-output", event);

    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = app_clone.emit(&output_event_name, line);
            }
        }
    });

    // 处理 stderr
    let error_event_name = format!("{}-error", event);
    let app_clone = app.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = app_clone.emit(&error_event_name, line);
            }
        }
    });
    Ok(cmd)
}
