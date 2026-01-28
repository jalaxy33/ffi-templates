use std::path::Path;

fn main() {
    check_python_venv();
    
    #[cfg(target_os = "windows")]
    link_windows_python_dll();
}

fn check_python_venv() {
    let venv_dir_str = std::env::var("VIRTUAL_ENV").unwrap_or(".venv".to_string());
    let venv_dir = Path::new(&venv_dir_str);
    let python_executable = if cfg!(windows) {
        &venv_dir.join("Scripts/python.exe")
    } else {
        &venv_dir.join("bin/python")
    };

    let should_init_venv = !venv_dir.exists() || !python_executable.exists();
    if should_init_venv {
        println!(
            "cargo:warning=Virtual environment not found at {:?}. Running `uv sync`...",
            venv_dir
        );

        // Activate virtual environment using `uv`
        std::process::Command::new("uv")
            .arg("sync")
            .status()
            .expect("Failed to execute `uv sync`");
    }

    println!("cargo:rerun-if-changed={}", venv_dir.to_str().unwrap());
    println!(
        "cargo:rerun-if-changed={}",
        python_executable.to_str().unwrap()
    );
}

#[cfg(target_os = "windows")]
fn link_windows_python_dll() {
    let venv_dir_str = std::env::var("VIRTUAL_ENV").unwrap_or_default();
    let venv_dir = Path::new(&venv_dir_str);
    let python_executable = &venv_dir.join("Scripts/python.exe");
    let python_exe_str = &python_executable.to_str().unwrap();

    check_python_venv();
    assert!(
        python_executable.exists(),
        "Python executable not found at {:?}",
        python_exe_str
    );

    // Add python DLL directory to PATH
    let output = std::process::Command::new(python_exe_str)
        .args(&["-c", "import sys; print(sys.base_exec_prefix)"])
        .output()
        .expect("Failed to execute python command to get executable path");
    match output.status.success() {
        true => {
            let py_dll_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path_env = std::env::var("PATH").unwrap_or_default();
            println!("cargo:rustc-env=PATH={};{}", path_env, py_dll_dir);
        }
        false => {
            println!("cargo:warning=Failed to get Python executable path");
        }
    }
}
