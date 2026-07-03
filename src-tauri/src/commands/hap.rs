use crate::hdc::client::HdcClient;
use crate::models::hap::{InstallStatus, InstallTask};
use tauri::State;

#[tauri::command]
pub fn install_hap(
    device_id: String,
    hap_path: String,
    hdc: State<'_, HdcClient>,
) -> Result<InstallTask, String> {
    let now = chrono_now();
    let mut task = InstallTask {
        device_id: device_id.clone(),
        hap_path: hap_path.clone(),
        status: InstallStatus::Installing,
        start_time: Some(now),
        end_time: None,
        error_message: None,
    };

    match hdc.install_hap(&device_id, &hap_path) {
        Ok(_output) => {
            task.status = InstallStatus::Success;
            task.end_time = Some(chrono_now());
            Ok(task)
        }
        Err(e) => {
            task.status = InstallStatus::Failed;
            task.end_time = Some(chrono_now());
            task.error_message = Some(e);
            Ok(task)
        }
    }
}

#[tauri::command]
pub fn validate_hap(hap_path: String, hdc: State<'_, HdcClient>) -> Result<(), String> {
    hdc.validate_hap_file(&hap_path)
}

fn chrono_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
