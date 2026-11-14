#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "Rust++",
        "version": env!("CARGO_PKG_VERSION"),
        "mode": "deterministic-offline",
        "languages": ["C++", "Rust", "Python", "JavaScript", "Java", "C#"]
    })
}

#[tauri::command]
fn generate_template(language: String) -> Result<rust_plus_plus::AssistantReply, String> {
    let lang = rust_plus_plus::parse_language(&language).map_err(|e| e.to_string())?;
    Ok(rust_plus_plus::knowledge::template(lang))
}

#[tauri::command]
fn explain_language(language: String) -> Result<rust_plus_plus::AssistantReply, String> {
    let lang = rust_plus_plus::parse_language(&language).map_err(|e| e.to_string())?;
    Ok(rust_plus_plus::knowledge::explain_language(lang))
}

#[tauri::command]
fn analyze_request(question: String) -> rust_plus_plus::AssistantReply {
    rust_plus_plus::engine::answer(&question)
}

#[tauri::command]
fn answer_common_question(
    question: String,
    language: String,
) -> Result<rust_plus_plus::AssistantReply, String> {
    let lang = rust_plus_plus::parse_language(&language).map_err(|e| e.to_string())?;
    rust_plus_plus::knowledge::answer_common_question(&question, lang)
        .ok_or_else(|| "That question is not in the built-in common questions library yet.".to_string())
}

#[tauri::command]
fn validate_source(language: String, source: String) -> Result<rust_plus_plus::AssistantReply, String> {
    let lang = rust_plus_plus::parse_language(&language).map_err(|e| e.to_string())?;
    Ok(rust_plus_plus::validate::lightweight_check(&source, lang))
}

#[tauri::command]
fn save_source_file(path: String, source: String) -> Result<String, String> {
    let path = std::path::PathBuf::from(path);
    if path.as_os_str().is_empty() { return Err("Choose a file path first.".into()); }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Could not create folder: {e}"))?;
    }
    std::fs::write(&path, source).map_err(|e| format!("Could not save file: {e}"))?;
    Ok(format!("Saved {}", path.display()))
}

#[tauri::command]
fn read_source_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Could not open file: {e}"))
}


#[tauri::command]
fn run_source(
    app: tauri::AppHandle,
    language: String,
    source: String,
) -> Result<serde_json::Value, String> {
    use std::{
        fs,
        path::PathBuf,
        process::{Command, Stdio},
        sync::atomic::{AtomicU64, Ordering},
        thread,
        time::{Duration, Instant},
    };

    static NEXT_ID: AtomicU64 = AtomicU64::new(1);

    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;

    if source.trim().is_empty() {
        return Err("There is no source code to run.".into());
    }
    if source.len() > 512_000 {
        return Err("Source file exceeds the 512 KB execution limit.".into());
    }

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let dir: PathBuf = std::env::temp_dir()
        .join(format!("rustpp-run-{}-{id}", std::process::id()));
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create temporary folder: {e}"))?;

    let result = (|| -> Result<serde_json::Value, String> {
        let (_file, compiler, run_program, run_args):
            (&str, Option<(String, Vec<String>)>, String, Vec<String>) =
        match language.as_str() {
            "cpp" => {
                let gcc_bin = resource_dir.join("binaries/cpp/mingw64/bin");
                let cc1plus_dir = resource_dir.join(
                    "binaries/cpp/mingw64/libexec/gcc/x86_64-w64-mingw32/16.2.0"
                );

                if cfg!(windows) {
                    let gcc = gcc_bin.join("g++.exe");
                    let cc1plus = cc1plus_dir.join("cc1plus.exe");

                    if !gcc.is_file() {
                        return Err(format!("Bundled g++.exe missing: {}", gcc.display()));
                    }

                    if !cc1plus.is_file() {
                        return Err(format!(
                            "Bundled cc1plus.exe missing: {} | Resource directory: {}",
                            cc1plus.display(),
                            resource_dir.display()
                        ));
                    }
                }

                let file = dir.join("main.cpp");
                fs::write(&file, &source).map_err(|e| e.to_string())?;
                let binary = dir.join(if cfg!(windows) { "program.exe" } else { "program" });
                ("main.cpp",
                 Some((if cfg!(windows) {
                     resource_dir.join("binaries/cpp/mingw64/bin/g++.exe").to_string_lossy().into_owned()
                 } else {
                     "g++".to_string()
                 }, vec![
                     "-isystem".into(),
                     resource_dir.join("binaries/cpp/mingw64/include/c++/16.2.0").to_string_lossy().into_owned(),
                     "-isystem".into(),
                     resource_dir.join("binaries/cpp/mingw64/include/c++/16.2.0/x86_64-w64-mingw32").to_string_lossy().into_owned(),
                     "-isystem".into(),
                     resource_dir.join("binaries/cpp/mingw64/lib/gcc/x86_64-w64-mingw32/16.2.0/include").to_string_lossy().into_owned(),
                     "-isystem".into(),
                     resource_dir.join("binaries/cpp/mingw64/lib/gcc/x86_64-w64-mingw32/16.2.0/include-fixed").to_string_lossy().into_owned(),
                     "-isystem".into(),
                     resource_dir.join("binaries/cpp/mingw64/include").to_string_lossy().into_owned(),
                     "-isystem".into(),
                     resource_dir.join("binaries/cpp/mingw64/x86_64-w64-mingw32/include").to_string_lossy().into_owned(),
                     file.to_string_lossy().into_owned(),
                     "-std=c++17".into(), "-O0".into(),
                     "-o".into(), binary.to_string_lossy().into_owned()
                 ])),
                 binary.to_string_lossy().into_owned(),
                 vec![])
            }
            "rust" => {
                let file = dir.join("main.rs");
                fs::write(&file, &source).map_err(|e| e.to_string())?;
                let binary = dir.join(if cfg!(windows) { "program.exe" } else { "program" });
                ("main.rs",
                 Some(("rustc".into(), vec![
                    file.to_string_lossy().into_owned(),
                    "-o".into(), binary.to_string_lossy().into_owned()
                 ])),
                 binary.to_string_lossy().into_owned(),
                 vec![])
            }
            "python" => {
                fs::write(dir.join("main.py"), &source).map_err(|e| e.to_string())?;
                ("main.py", None, if cfg!(windows) { "python" } else { "python3" }.into(),
                 vec!["main.py".into()])
            }
            "javascript" => {
                fs::write(dir.join("main.js"), &source).map_err(|e| e.to_string())?;
                ("main.js", None, "node".into(), vec!["main.js".into()])
            }
            "java" => {
                fs::write(dir.join("Main.java"), &source).map_err(|e| e.to_string())?;
                ("Main.java",
                 Some(("javac".into(), vec!["Main.java".into()])),
                 "java".into(), vec!["Main".into()])
            }
            "csharp" => {
                fs::write(dir.join("Program.cs"), &source).map_err(|e| e.to_string())?;
                let project = r#"<Project Sdk="Microsoft.NET.Sdk">
<PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings></PropertyGroup>
</Project>"#;
                fs::write(dir.join("RustPP.csproj"), project).map_err(|e| e.to_string())?;
                ("Program.cs",
                 Some(("dotnet".into(), vec![
                    "build".into(), "RustPP.csproj".into(), "--nologo".into(),
                    "--verbosity".into(), "quiet".into()
                 ])),
                 "dotnet".into(), vec![
                    "run".into(), "--project".into(), "RustPP.csproj".into(),
                    "--no-build".into()
                 ])
            }
            _ => return Err("Unsupported language.".into()),
        };

        let execute = |program: &str, arguments: &[String]| -> Result<(i32, String, String), String> {
            let stdout_path = dir.join("stdout.txt");
            let stderr_path = dir.join("stderr.txt");
            let stdout = fs::File::create(&stdout_path).map_err(|e| e.to_string())?;
            let stderr = fs::File::create(&stderr_path).map_err(|e| e.to_string())?;

            let mut command = Command::new(program);

            if cfg!(windows) {
                let mut paths = vec![
                    resource_dir.join("binaries/cpp/mingw64/bin"),
                    resource_dir.join(
                        "binaries/cpp/mingw64/libexec/gcc/x86_64-w64-mingw32/16.2.0"
                    ),
                ];
                if let Some(existing) = std::env::var_os("PATH") {
                    paths.extend(std::env::split_paths(&existing));
                }
                if let Ok(path) = std::env::join_paths(paths) {
                    command.env("PATH", path);
                }
            }



            let mut child = command
                .args(arguments)
                .current_dir(&dir)
                .stdin(Stdio::null())
                .stdout(Stdio::from(stdout))
                .stderr(Stdio::from(stderr))
                .spawn()
                .map_err(|e| format!("Could not start '{program}': {e}. Check that its toolchain is installed and on PATH."))?;

            let start = Instant::now();
            let timeout = Duration::from_secs(10);
            let status = loop {
                if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
                    break status;
                }
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("Execution timed out after {} seconds.", timeout.as_secs()));
                }
                thread::sleep(Duration::from_millis(40));
            };

            let mut out = fs::read_to_string(stdout_path).unwrap_or_default();
            let mut err = fs::read_to_string(stderr_path).unwrap_or_default();
            out.truncate(64_000);
            err.truncate(64_000);
            Ok((status.code().unwrap_or(-1), out, err))
        };

        if let Some((program, arguments)) = compiler {
            let (code, out, err) = execute(&program, &arguments)?;
            if code != 0 {
                let explanation: Vec<String> = [err, out]
                    .into_iter()
                    .filter(|text| !text.is_empty())
                    .collect();

                return Ok(serde_json::json!({
                    "source": "RUST++ COMPILER",
                    "title": "Compilation failed",
                    "message": format!("Compiler exited with code {code}."),
                    "explanation": explanation
                }));
            }
        }

        let (code, out, err) = execute(&run_program, &run_args)?;
        Ok(serde_json::json!({
            "source": "RUST++ LOCAL EXECUTION",
            "title": if code == 0 { "Program finished" } else { "Program exited with an error" },
            "message": format!("Exit code: {code}"),
            "code": if out.is_empty() { "(No standard output)" } else { &out },
            "explanation": if err.is_empty() { vec!["Execution completed locally.".to_string()] } else { vec![format!("Standard error:\n{err}")] }
        }))
    })();

    let _ = fs::remove_dir_all(&dir);
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_info, generate_template, explain_language, analyze_request, answer_common_question,
            validate_source, save_source_file, read_source_file, run_source
        ])
        .run(tauri::generate_context!())
        .expect("error while running Rust++");
}
