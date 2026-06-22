use std::process::Command;

#[tauri::command]
fn ejecutar_script(nombre_archivo: String) -> Result<String, String> {
    println!("Rust ejecutando index.ts para el archivo: {}", nombre_archivo);
    
    let ruta_index = "C:\\Users\\USUARIO\\Desktop\\haxball-converter-hbr2-to-mp4-software";

    let output = Command::new("cmd")
        .current_dir(ruta_index)
        .args(["/C", "npx", "tsx", "index.ts", &nombre_archivo])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                Ok(stdout)
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Err(format!("Error del script: {}", stderr))
            }
        }
        Err(e) => Err(format!("No se pudo levantar Node: {}", e.to_string())),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ejecutar_script])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
