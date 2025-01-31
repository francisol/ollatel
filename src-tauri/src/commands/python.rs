use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Cursor, Write};
// src-tauri/src/main.rs
use serde::Deserialize;
use std::path::PathBuf;
use tauri::http::request::{Builder, Request};
use tauri::path::BaseDirectory;
use tauri::{Emitter, Manager};
use wmi::{COMLibrary, Variant, WMIConnection};

use super::run_command;

#[tauri::command]
pub async fn install_python<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let python_dir = app.path().app_data_dir().unwrap().join("python");
    let _ = fs::create_dir_all(&python_dir);

    let zip_path = app
        .path()
        .resolve("resources/python.zip", BaseDirectory::Resource)
        .unwrap();
    let data = fs::read(zip_path).map_err(|e| e.to_string())?;

    // 解压ZIP文件
    zip_extract::extract(Cursor::new(data), &python_dir, false).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn install_pip<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let python_dir = app.path().app_data_dir().unwrap().join("python");
    let get_pip = app
        .path()
        .resolve("resources/get-pip.py", BaseDirectory::Resource)
        .unwrap();
    let pip_path = python_dir.join("get-pip.py");
    fs::copy(get_pip, pip_path).map_err(|e| e.to_string())?;
    // let path_path = (&python_dir).join("python.exe");
    run_command(
        &app,
        "python.exe",
        vec!["get-pip.py"],
        &python_dir,
        "install-pip",
        HashMap::new(),
    )?;

    let path_path = (&python_dir).join("python310._pth");
    let mut file = OpenOptions::new()
        .append(true)
        .open(path_path)
        .map_err(|e| e.to_string())?;
    writeln!(&mut file, "import site");
    Ok(())
}

// fn run_python<R: tauri::Runtime, S: AsRef<OsStr>,I: IntoIterator<Item = S>>(app: tauri::AppHandle<R>, args: I) -> Result<(), String> {
//     let python_dir = app.path().app_data_dir().unwrap().join("python");
//     run_command(&app, "python.exe", args, python_dir, "run-python",HashMap::new())
// }
// fn run_pip<R: tauri::Runtime, S: AsRef<OsStr>,I: IntoIterator<Item = S>>(app: tauri::AppHandle<R>, args:I) -> Result<(), String> {
//     let python_dir = app.path().app_data_dir().unwrap().join("python");
//     run_command(&app, "pip", args, python_dir, "run-pip",HashMap::new())
// }

#[tauri::command]
pub async fn install_deps<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    // let com_con = COMLibrary::new()        .map_err(|e| e.to_string())?;

    // let wmi_con = WMIConnection::new(com_con.into())        .map_err(|e| e.to_string())?;

    // println!("查询显卡信息");

    // // 查询显卡信息
    // let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_VideoController").map_err(|e| e.to_string())?;

    // for (index, gpu) in results.iter().enumerate() {
    //    match &gpu["VideoProcessor"] {
    //        Variant::Empty => {},
    //        Variant::Null => {},
    //        Variant::String(process) => {
    //             if process.starts_with("Intel(R) Arc(TM)") {
    //               return   run_python(app.clone(),&vec!["-m","pip","install","--pre","--upgrade","ipex-llm[cpp]"]);
    //             }
    //        },
    //        Variant::I1(_) => {},
    //        Variant::I2(_) => {},
    //        Variant::I4(_) => {},
    //        Variant::I8(_) => {},
    //        Variant::R4(_) => {},
    //        Variant::R8(_) => {},
    //        Variant::Bool(_) => {},
    //        Variant::UI1(_) => {},
    //        Variant::UI2(_) => {},
    //        Variant::UI4(_) => {},
    //        Variant::UI8(_) => {},
    //        Variant::Array(_) => {},
    //        Variant::Unknown(_) => {},
    //        Variant::Object(_) => {},
    //    }
    // }
    let python_dir = app.path().app_data_dir().unwrap().join("python");
    run_command(
        &app,
        "pip",
        vec!["install", "--pre", "--upgrade", "ipex-llm[cpp]"],
        python_dir,
        "run-pip",
        HashMap::new(),
    )

}
