use std::env;
use std::path::Path;

fn main() {
    #[cfg(target_os = "windows")]
    link_windows_python_dll();
}

#[cfg(target_os = "windows")]
fn link_windows_python_dll() {
    let venv_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(".venv");
    let python_executable = venv_dir.join("Scripts/python.exe");

    let venv_dir_str = &venv_dir.to_str().unwrap().to_string();
    let python_path_str = &python_executable.to_str().unwrap().to_string();

    if !python_executable.exists() {
        println!(
            "cargo:warning=Python executable not found under {}, trying to install python dependencies using 'uv sync'",
            venv_dir_str
        );
        std::process::Command::new("uv")
            .args(&["sync"])
            .output()
            .expect("Failed to execute uv sync command");
    }

    assert!(
        python_executable.exists(),
        "Python executable not found at {}",
        python_path_str
    );

    let output = std::process::Command::new(python_path_str)
        .args(&["-c", "import sys; print(sys.base_exec_prefix)"])
        .output()
        .expect("Failed to execute python command to get executable path");
    match output.status.success() {
        true => {
            let py_dll_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path_env = env::var("PATH").unwrap_or_default();
            println!("cargo:rustc-env=PATH={};{}", path_env, py_dll_dir);
        }
        false => {
            println!("cargo:warning=Failed to get Python executable path");
        }
    }
}

